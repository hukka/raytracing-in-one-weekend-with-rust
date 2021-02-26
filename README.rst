Install
=======
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Run in dev mode
===============
`cargo run -- > test.ppm` for a PPM ASCII image,
`cargo run -- -b > test.ppm` for a PPM binary image,
`cargo run -- -w` for a Vulkan rendered window.

Build
=====
`cargo b --release && strip target/release/raytracing`

Uninstall
=========
`rustup self uninstall`
