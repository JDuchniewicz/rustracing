use crate::vec3::Color;
use std::io;
use std::io::Write;

pub fn write_color(
    stream: &mut impl Write,
    pixel_color: Color,
    samples_per_pixel: i32,
) -> Result<(), io::Error> {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    writeln!(
        stream,
        "{} {} {}",
        ((r * (u8::MAX as f64 + 1.)) as i32).clamp(0, u8::MAX as i32),
        ((g * (u8::MAX as f64 + 1.)) as i32).clamp(0, u8::MAX as i32),
        ((b * (u8::MAX as f64 + 1.)) as i32).clamp(0, u8::MAX as i32)
    )
    .map(|_| ())
}
