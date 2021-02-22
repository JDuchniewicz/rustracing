use crate::vec3::Color;
use std::io::Write;

pub fn write_color(stream: &mut impl Write, color: Color) -> () {
    let ir = (255.999 * color.x) as i32;
    let ig = (255.999 * color.y) as i32;
    let ib = (255.999 * color.z) as i32;

    stream.write_fmt(format_args!("{} {} {}\n", ir, ig, ib));
}
