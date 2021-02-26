// TODO
//
// Ray tracing:
// - first rays
// - global illumination with diffusion
// - GI with specularity
//
// Path tracing:
// - reflections (one bounce)
// - diffusion (random bounce)
// - opacity / diffraction
// - ray perturbation (soft edges / anti aliasing)
// - light sources
// - textured surfaces
// - simple non-spherical geometries
// - loading and rendering 3D models
// - accumulating rays (rendering a noisy version first and improving from there)
// - Depth of Field
//
// Yak shaving:
// - choosing float precision at runtime
// - multithreading
// - nalgebra version
// - ultraviolet version?
// - microbenchmarks with criterion
// - handling cli arguments with that crate I don't remember now
// - change settings at runtime with keybinds
// - wasm version for web
// - Vulkan raytracing APIs (https://github.com/GPSnoopy/RayTracingInVulkan,
//                           https://github.com/vaffeine/vulkano-raytracing)
// - stereoscopic views (see https://www.iquilezles.org/www/index.htm)
// - moving camera + other controls (viewport size, focal distance)
// - saving midpoint as image
// - saving rendering process as video/gif
// - fancy compression for images and video
// - GUI for settings with dear imgui

use std::io::Write;

use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod vec3;
use vec3::Vec3;

const WIDTH: u16 = 255;
const HEIGHT: u16 = 255;
const SCALE: u8 = 255;

struct Ray {
    origin: Vec3,
    direction: Vec3,
}

struct Sphere {
    radius: f32,
    position: Vec3,
}

#[test]
fn test_sphere_intersections() {
    let s = Sphere { radius: 1.0, position: Vec3::new(0.0, 0.0, 0.0) };

    // Ray intersects
    assert_eq!(
        s.intersect_t(&Ray {
            origin: Vec3::new(-10.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 0.0, 0.0),
        }),
        Some(9.0));

    // Ray points wrong way
    assert_eq!(
        s.intersect_t(&Ray {
            origin: Vec3::new(-10.0, 0.0, 0.0),
            direction: Vec3::new(-1.0, 0.0, 0.0),
        }),
        None);

    // Ray starts inside
    assert_eq!(
        s.intersect_t(&Ray {
            origin: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 0.0, 0.0),
        }),
        Some(1.0));

    // Ray hits edge
    assert_eq!(
        s.intersect_t(&Ray {
            origin: Vec3::new(-10.0, 1.0, 0.0),
            direction: Vec3::new(1.0, 0.0, 0.0),
        }),
        Some(10.0));

    // Ray misses
    assert_eq!(
        s.intersect_t(&Ray {
            origin: Vec3::new(-10.0, 2.0, 0.0),
            direction: Vec3::new(1.0, 0.0, 0.0),
        }),
        None);

    assert_eq!(
        s.intersect(&Ray {
            origin: Vec3::new(-10.0, 0.0, 0.0),
            direction: Vec3::new(1.0, 0.0, 0.0),
        }),
        Some(Vec3::new(-1.0, 0.0, 0.0)));
    assert_eq!(
        s.intersect(&Ray {
            origin: Vec3::new(-10.0, 1.0, 0.0),
            direction: Vec3::new(1.0, 0.0, 0.0),
        }),
        Some(Vec3::new(0.0, 1.0, 0.0)));
}

impl Sphere {
    fn intersect_t(&self, r: &Ray) -> Option<f32> {
        // If a point P is on the sphere S with radius r, centered at origin,
        // it must satisfy:
        //   ‖𝐏‖ = r
        // that is:
        //   P_x² + P_y² + P_z² = r²
        //
        // If the sphere is not at origin:
        //   ‖𝐏-𝐒‖ = r
        //   (P_x-S_x)² + (P_y-S_y)² + (P_z-S_z)² = r²
        //   (𝐏-𝐒) ⋅ (𝐏-𝐒) = r²
        //
        // A Point P(t) is on the Ray R with origin O and direction D, if for some t≥0:
        //  𝐎 + t𝐃 = 𝐏(t), so  O_c + tD_c = P_c, for every coordinate c∈{x, y, z}
        //
        // Therefore a ray on the sphere needs to satisfy
        //   (𝐎 + t𝐃 − 𝐒)⋅(𝐎 + t𝐃 − 𝐒) = r²
        //   (𝐃 ⋅ 𝐃)t² + 2(𝐃 ⋅ (𝐎 − 𝐒))t + (𝐎 − 𝐒)⋅(𝐎 − 𝐒) − r² = 0
        //
        // Quadratic polynomials (ax²+bx+c=0) are solved by:
        //   x = (-b±√(b²-4ac)) / 2a
        // where the part inside the square root is called the discriminant.

        // (𝐎 − 𝐒)
        let os = r.origin - self.position;

        // a = 𝐃 ⋅ 𝐃, which is always positive
        let a = r.direction.dot(r.direction);

        // b = 2(𝐃 ⋅ (𝐎 − 𝐒))
        let b = 2.0 * r.direction.dot(os);

        //   c = (𝐎 − 𝐒)⋅(𝐎 − 𝐒) − r²
        let c = os.dot(os) - self.radius * self.radius;
        let discriminant = b*b - 4.0*a*c;

        // The discriminant determines whether there is a solution, or more precisely
        // if the ray misses the sphere (negative discriminant, giving an imaginary result),
        if discriminant < 0.0 {
            return None;
        }

        // if it hits an edge (zero),
        let t1 = (-b - discriminant.sqrt()) / (2.0*a);
        if t1 >= 0.0 {
            // (since a is always positive, t1≤t2 always,
            //  so if t1 is not negative, it's the only or closer solution)
            return Some(t1);
        }

        // or it goes through (positive, giving two real results).
        let t2 = (-b + discriminant.sqrt()) / (2.0*a);
        if t2 > 0.0 {
            return Some(t2);
        }

        // If both solutions for t are negative,
        // the ray is pointing the wrong way (hit point is "behind" the origin of ray)
        None
    }

    fn intersect(&self, r: &Ray) -> Option<Vec3> {
        match self.intersect_t(r) {
            None => None,
            Some(t) => Some(r.origin + r.direction*t),
        }
    }
}

struct Camera {
    position: Vec3,
    direction: Vec3,
    height: Vec3,
}

impl Camera {
    fn new(position: Vec3, direction: Vec3, height: Vec3) -> Camera {
        Camera {
            position,
            direction,
            height,
        }
    }

    // Vector of same length and perpendicular to both height and direction.
    fn width(&self) -> Vec3 {
        let w = self.direction.cross(self.height);
        w / w.length() * self.height.length()
    }

    fn ray(&self, x: u16, y: u16) -> Ray {
        let vertical_offset = (x as f32 / WIDTH as f32) - 0.5;
        let horizontal_offset = (y as f32 / HEIGHT as f32) - 0.5;
        return Ray {
            origin: self.position,
            direction: self.direction + self.height * vertical_offset + self.width() * horizontal_offset,
        };
    }
}

fn arg(target : &str) -> bool {
    return std::env::args().any(|x| x == target);
}

fn get_gradient_color(x: u16, y: u16) -> [u8; 4] {
    return [(x*SCALE as u16/WIDTH) as u8, (y*SCALE as u16/HEIGHT) as u8, 0, 255];
}

fn get_raydistance_color(x: u16, y: u16) -> [u8; 4] {
    let camera = Camera::new(
        Vec3::new(-10.0, 0.0, 0.0),
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        );
    let sphere = Sphere { radius: 1.0, position: Vec3::new(10.0, 0.0, 0.0) };
    match sphere.intersect_t(&camera.ray(x, y)) {
        None => get_gradient_color(x, y),
        Some(t) => [((t-1.9)*1000.0) as u8, 0, 0, 255],
    }
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
                std::io::stdout().write(&get_raydistance_color(i, j)[..3]).unwrap();
            } else {
                let [r, g, b, _] = get_raydistance_color(i, j);
                println!("{} {} {}", r, g, b);
            }
        }
    }
}

fn main() -> Result<()> {
    let binary = arg("-b");
    let usewinit = arg("-w");

    if usewinit {
        run_winit()?;
    } else {
        write_ppm(binary);
    }
    Ok(())
}

fn run_winit() -> Result<()> {
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
                let frame = pixels.get_frame();
                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = (i % WIDTH as usize) as u16;
                    let y = (i / WIDTH as usize) as u16;

                    pixel.copy_from_slice(&get_raydistance_color(x, y));
                }
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
