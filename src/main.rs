use anyhow::Result;
use ash::{self, vk};

fn main() -> Result<()> {
    let entry: ash::Entry = unsafe { ash::Entry::load()}?;
    let application_info: vk::ApplicationInfoBuilder<'_> = vk::ApplicationInfo::builder().api_version(vk::API_VERSION_1_3);
    let create_info: vk::InstanceCreateInfoBuilder<'_> = vk::InstanceCreateInfo::builder().application_info(&application_info);
    let instance: ash::Instance = unsafe {entry.create_instance(&create_info, None)}?;

    unsafe {instance.destroy_instance(None)}
    Ok(())
}
