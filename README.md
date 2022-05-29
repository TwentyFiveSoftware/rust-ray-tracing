# Ray Tracing

<img src="https://github.com/TwentyFiveSoftware/ray-tracing-gpu/blob/master/sceneRender.png">

## Overview

This is my take on [Peter Shirley's Ray Tracing in One Weekend](https://github.com/RayTracing/raytracing.github.io) book.

This project uses the [Rust](https://www.rust-lang.org/) programming language to achieve high performance and memory efficiency while providing many benefits such as memory- and thread-safety.

## Build & Run this project

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Clone the repository
3. Build the project
   ```sh
   cargo build --release
   ```
4. Run the executable
   ```sh
   ./target/release/rust-ray-tracing
   ```
   
   Display command line options
   ```sh
   ./target/release/rust-ray-tracing --help
   ```

## Performance

I've already implemented Peter Shirley's ray tracing in various programming languages running on CPU & GPU and compared their performance.

The performance was measured on the same scene (see image above) with the same amount of objects, the same recursive
depth, the same resolution (1920 x 1080). The measured times are averaged over multiple runs.

*Reference system: AMD Ryzen 9 5900X (12 Cores / 24 Threads) | AMD Radeon RX 6800 XT*

|                                                                                                                     | 1 sample / pixel | 100 samples / pixel |        10,000 samples / pixel | 
|---------------------------------------------------------------------------------------------------------------------|-----------------:|--------------------:|------------------------------:|
| [CPU Ray Tracing <br/>(C++)](https://github.com/TwentyFiveSoftware/ray-tracing)                                     |       1,300.0 ms |             125.2 s | ~ 3.5 h <br/>_(extrapolated)_ |
| [CPU Ray Tracing <br/>(Rust)](https://github.com/TwentyFiveSoftware/rust-ray-tracing)                               |         500.0 ms |              49.7 s | ~ 1.4 h <br/>_(extrapolated)_ |
| [GPU Ray Tracing <br/>(Compute Shader)](https://github.com/TwentyFiveSoftware/ray-tracing-gpu)                      |          21.5 ms |               2.1 s |                       215.0 s |
| [GPU Ray Tracing <br/>(Vulkan Ray Tracing extension)](https://github.com/TwentyFiveSoftware/ray-tracing-gpu-vulkan) |           1.2 ms |               0.1 s |                        12.5 s |
