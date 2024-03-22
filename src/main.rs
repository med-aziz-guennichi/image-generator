use anyhow::{Context, Result};
use ash::{self,vk::{self, DeviceQueueCreateInfo}};
use gpu_allocator::{vulkan::*, MemoryLocation};
use softbuffer::GraphicsContext;
use std::env;
use std::time;
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::WindowBuilder};

use anyhow::anyhow;

fn main() -> Result<()> {
    // Config
    let mut args: std::iter::Skip<env::Args> = env::args().skip(1);
    let width: u32 = args.next().context("width is required")?.parse::<u32>()?;
    let height: u32 = args.next().context("height is required")?.parse::<u32>()?;
    let mut red: u32 = args.next().context("red is required")?.parse::<u32>()?;
    let green: u32 = args.next().context("green is required")?.parse::<u32>()?;
    let blue: u32 = args.next().context("blue is required")?.parse::<u32>()?;

    let event_loop: EventLoop<()> = EventLoop::new();
    let window: winit::window::Window = WindowBuilder::new().with_title("CODOTAKU").with_inner_size(PhysicalSize::new(width, height)).with_resizable(false).build(&event_loop)?;

    let mut graphics_context: GraphicsContext =
        unsafe { GraphicsContext::new(&window, &window) }.map_err(|error| anyhow!("{error}"))?;

    let entry: ash::Entry = unsafe { ash::Entry::load() }?;

    let instance: ash::Instance = {
        let application_info: vk::ApplicationInfoBuilder<'_> = vk::ApplicationInfo::builder().api_version(vk::API_VERSION_1_3);
        let create_info: vk::InstanceCreateInfoBuilder<'_> = vk::InstanceCreateInfo::builder().application_info(&application_info);
        unsafe { entry.create_instance(&create_info, None) }?
    };

    let physical_device: vk::PhysicalDevice = unsafe { instance.enumerate_physical_devices() }?.into_iter().next().context("No physical device found")?;

    let queue_family_index: u32 = {
        let mut queue_families_properties: Vec<(usize, vk::QueueFamilyProperties)> =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) }.into_iter().enumerate().filter(|queue_family_properties| {
                    queue_family_properties.1.queue_flags.intersects(
                        vk::QueueFlags::TRANSFER
                            | vk::QueueFlags::GRAPHICS
                            | vk::QueueFlags::COMPUTE,
                    )
                })
                .collect::<Vec<_>>();
        queue_families_properties.sort_by_key(|queue_family_properties: &(usize, vk::QueueFamilyProperties)| {
            (
                queue_family_properties.1.queue_flags.as_raw().count_ones(),
                queue_family_properties.1.queue_count,
            )
        });
        queue_families_properties
            .first()
            .context("No suitable queue family")?
            .0 as u32
    };

    println!("{queue_family_index}");

    let device: ash::Device = {
        let queue_priorities: [f32; 1] = [1.0];
        let queue_create_info: vk::DeviceQueueCreateInfoBuilder<'_> = DeviceQueueCreateInfo::builder().queue_family_index(queue_family_index).queue_priorities(&queue_priorities);

        let create_info: vk::DeviceCreateInfoBuilder<'_> = vk::DeviceCreateInfo::builder().queue_create_infos(std::slice::from_ref(&queue_create_info));
        unsafe { instance.create_device(physical_device, &create_info, None) }?
    };

    let queue: vk::Queue = unsafe { device.get_device_queue(queue_family_index, 0) };

    let mut t: f64 = 0.0;
    {
        // Create allocator
        let mut allocator: Option<Allocator> = Option::Some({
            let allocator_create_description: AllocatorCreateDesc = AllocatorCreateDesc {
                instance: instance.clone(),
                device: device.clone(),
                physical_device,
                debug_settings: Default::default(),
                buffer_device_address: false,
            };
            Allocator::new(&allocator_create_description)?
        });

        let value_count = width * height;

        // Create buffer
        let buffer: vk::Buffer = {
            let create_info: vk::BufferCreateInfoBuilder<'_> = vk::BufferCreateInfo::builder().size(value_count as vk::DeviceSize * std::mem::size_of::<u32>() as vk::DeviceSize).usage(vk::BufferUsageFlags::TRANSFER_DST);
            unsafe { device.create_buffer(&create_info, None) }?
        };

        let mut allocation: Option<Allocation> = Option::Some({
            let memory_requirements: vk::MemoryRequirements = unsafe { device.get_buffer_memory_requirements(buffer) };

            let allocation_create_description: AllocationCreateDesc<'_> = AllocationCreateDesc {
                name: "Buffer allocation",
                requirements: memory_requirements,
                location: MemoryLocation::GpuToCpu,
                linear: true,
            };

            let allocation: Allocation = allocator.as_mut().unwrap().allocate(&allocation_create_description)?;
            unsafe { device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset()) }?;
            allocation
        });

        let command_pool: vk::CommandPool = {
            let create_info: vk::CommandPoolCreateInfoBuilder<'_> = vk::CommandPoolCreateInfo::builder().queue_family_index(queue_family_index).flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
            unsafe { device.create_command_pool(&create_info, None) }?
        };

        let command_buffer: vk::CommandBuffer = {
            let create_info: vk::CommandBufferAllocateInfoBuilder<'_> = vk::CommandBufferAllocateInfo::builder().level(vk::CommandBufferLevel::PRIMARY).command_pool(command_pool).command_buffer_count(1);
            unsafe { device.allocate_command_buffers(&create_info) }?.into_iter().next().context("No command buffer found")?
        };

        // Creating synchronization object (Fence)
        let fence: vk::Fence = {
            let create_info: vk::FenceCreateInfo = vk::FenceCreateInfo::builder()
                .flags(vk::FenceCreateFlags::SIGNALED)
                .build();
            unsafe { device.create_fence(&create_info, None) }?
        };

        event_loop.run(move |event: winit::event::Event<'_, ()>, _, control_flow: &mut winit::event_loop::ControlFlow| match event {
            winit::event::Event::WindowEvent { window_id, event } => {
                if window_id == window.id() {
                    match event {
                        winit::event::WindowEvent::CloseRequested => {
                            control_flow.set_exit();
                        }
                        _ => {}
                    }
                }
            }
            winit::event::Event::MainEventsCleared => {
                let start: time::Instant = time::Instant::now();
                t += 0.001;
                red = ((t.sin() * 0.5 + 0.5) * 255.0) as u32;
                // Wait for the execution to complete
                unsafe { device.wait_for_fences(std::slice::from_ref(&fence), true, u64::MAX) }
                    .unwrap();
                unsafe { device.reset_fences(std::slice::from_ref(&fence)) }.unwrap();

                // Recording command buffer
                {
                    let begin_info: vk::CommandBufferBeginInfoBuilder<'_> = vk::CommandBufferBeginInfo::builder();
                    unsafe { device.begin_command_buffer(command_buffer, &begin_info) }.unwrap();
                }

                let value = blue | green << 8 | red << 16;
                unsafe {
                    device.cmd_fill_buffer(command_buffer,buffer,allocation.as_ref().unwrap().offset(),allocation.as_ref().unwrap().size(),value)
                }

                unsafe { device.end_command_buffer(command_buffer) }.unwrap();

                // Execute command buffer by uploading it to the GPU through the queue
                {
                    let submit_info: vk::SubmitInfoBuilder<'_> = vk::SubmitInfo::builder()
                        .command_buffers(std::slice::from_ref(&command_buffer));
                    unsafe {
                        device.queue_submit(queue, std::slice::from_ref(&submit_info), fence)
                    }
                    .unwrap();
                }

                let data: &[u32] = bytemuck::cast_slice(
                    allocation.as_ref().unwrap().mapped_slice().context("Cannot access buffer from Host").unwrap(),
                );

                graphics_context.set_buffer(data, width as u16, height as u16);
                println!("{:?}", time::Instant::now() - start);
            }
            winit::event::Event::LoopDestroyed => {
                unsafe { device.queue_wait_idle(queue) }.unwrap();

                unsafe { device.destroy_fence(fence, None) }
                unsafe { device.destroy_command_pool(command_pool, None) }

                allocator.as_mut().unwrap().free(allocation.take().unwrap()).unwrap();
                drop(allocator.take().unwrap());
                unsafe { device.destroy_buffer(buffer, None) }

                unsafe { device.destroy_device(None) }
                unsafe { instance.destroy_instance(None) }
            }
            _ => {}
        });
    }
}
