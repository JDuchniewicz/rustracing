mod color;
mod vec3;

use color::write_color;
use std::io::{self, Write};
use vec3::{Color, Vec3};

fn main() {
    // TODO: error handling
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_height, image_width);

    for j in (0..=image_height - 1).into_iter().rev() {
        eprintln!("Scanlines remaining {}", j);
        for i in 0..image_width {
            let pixel_color: Color = Color::new().with_values(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );
            write_color(&mut handle, pixel_color);
        }
    }
    eprintln!("Done");
}
