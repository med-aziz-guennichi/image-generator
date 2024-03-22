# Vulkan Graphics Renderer

![GitHub repo size](https://img.shields.io/github/repo-size/med-aziz-guennichi/vulkan-graphics-renderer)
![GitHub contributors](https://img.shields.io/github/contributors/med-aziz-guennichi/vulkan-graphics-renderer)
![GitHub stars](https://img.shields.io/github/stars/med-aziz-guennichi/vulkan-graphics-renderer?style=social)
![GitHub forks](https://img.shields.io/github/forks/med-aziz-guennichi/vulkan-graphics-renderer?style=social)

A simple graphics renderer using Vulkan API for rendering graphics.

## Prerequisites

Before you begin, ensure you have met the following requirements:
* Rust and Cargo installed on your local machine
* Vulkan SDK installed
* GPU that supports Vulkan

## Installing Vulkan Graphics Renderer

To install vulkan-graphics-renderer, follow these steps:

Linux and macOS:
`
git clone https://github.com/your-username/vulkan-graphics-renderer.git
cd vulkan-graphics-renderer
cargo build
`

## Using Vulkan Graphics Renderer

To use vulkan-graphics-renderer, follow these steps:

1. Clone the repository:
    ```bash
    git clone https://github.com/your-username/vulkan-graphics-renderer.git
    cd vulkan-graphics-renderer
    ```

2. Build the project:
    ```bash
    cargo build
    ```

3. Run the program with the desired width, height, and color parameters:
    ```bash
    cargo run <width> <height> <red> <green> <blue>
    ```

Replace `<width>`, `<height>`, `<red>`, `<green>`, and `<blue>` with the desired values for width, height, and color components. For example:
    ```bash
    cargo run 800 600 255 0 0
    ```
This command will run the program with a width of 800 pixels, height of 600 pixels, and the color red (255, 0, 0).

## Contributing to Vulkan Graphics Renderer

To contribute to Vulkan Graphics Renderer, follow these steps:

1. Fork this repository.
2. Create a branch: `git checkout -b feature/feature-name`.
3. Make your changes and commit them: `git commit -m 'feature description'`.
4. Push to the original branch: `git push origin feature/feature-name`.
5. Create a pull request.

Alternatively, see the GitHub documentation on [creating a pull request](https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request).

## Contact

If you have any questions or concerns, please don't hesitate to contact me at medazizguennichi@gmail.com.
