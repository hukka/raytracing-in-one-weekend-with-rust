use std::io::Write;

fn arg(target : &str) -> bool {
    return std::env::args().any(|x| x == target);
}

fn main() {
    let binary = arg("-b");
    let width = 255;
    let height = 255;
    let scale = 255;

    if binary {
        println!("P6");
    } else {
        println!("P3");
    }
    println!("{} {}", width, height);
    println!("{}", scale);

    for i in 0..height {
        for j in 0..width {
            if binary {
                std::io::stdout().write(&[i, j, 0]).unwrap();
            } else {
                println!("{} {} {}", i, j, 0);
            }
        }
    }
}
