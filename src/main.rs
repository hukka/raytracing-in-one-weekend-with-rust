use std::io::Write;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

const WIDTH: u16 = 255;
const HEIGHT: u16 = 255;
const SCALE: u8 = 255;

fn arg(target : &str) -> bool {
    return std::env::args().any(|x| x == target);
}

fn get_color(x: u16, y: u16) -> [u8; 4] {
    return [(x*SCALE as u16/WIDTH) as u8, (y*SCALE as u16/HEIGHT) as u8, 0, 255];
}

fn write_ppm(binary : bool) {
    if binary {
        println!("P6");
    } else {
        println!("P3");
    }
    println!("{} {}", WIDTH, HEIGHT);
    println!("{}", SCALE);

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if binary {
                std::io::stdout().write(&get_color(i, j)[..3]).unwrap();
            } else {
                let [r, g, b, _] = get_color(i, j);
                println!("{} {} {}", r, g, b);
            }
        }
    }
}

fn main() {
    let binary = arg("-b");
    let usewinit = arg("-w");

    if usewinit {
        run_winit();
    } else {
        write_ppm(binary);
    }
}

fn run_winit() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Raytracing in one weekend")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            },
            Event::MainEventsCleared => {
                draw(pixels.get_frame());
                if pixels.render().is_err() {
                    println!("EEEK!");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            },
            _ => ()
        }
    });
}

fn draw(frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % WIDTH as usize) as u16;
        let y = (i / WIDTH as usize) as u16;

        pixel.copy_from_slice(&get_color(x, y));
    }
}
