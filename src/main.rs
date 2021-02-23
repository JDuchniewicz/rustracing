mod color;
mod ray;
mod vec3;

use color::write_color;
use ray::Ray;
use std::io::{self, Write};
use vec3::{Color, Point3, Vec3};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = Vec3::dot(&ray.direction, &ray.direction);
    let b = 2.0 * Vec3::dot(&oc, &ray.direction);
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn ray_color(ray: Ray) -> Color {
    if hit_sphere(&Point3::with_values(0.0, 0.0, -1.0), 0.5, &ray) {
        return Color::with_values(1.0, 0.0, 0.0);
    }
    let unit_direction: Vec3 = Vec3::unit_vector(&ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::with_values(1.0, 1.0, 1.0) + t * Color::with_values(0.5, 0.7, 1.0)
}

fn main() {
    // TODO: error handling
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::with_values(0.0, 0.0, 0.0);
    let horizontal = Vec3::with_values(viewport_width, 0.0, 0.0);
    let vertical = Vec3::with_values(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::with_values(0.0, 0.0, focal_length);

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..=image_height - 1).into_iter().rev() {
        eprintln!("Scanlines remaining {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::with_values(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color: Color = ray_color(r);
            write_color(&mut handle, pixel_color);
        }
    }
    eprintln!("Done");
}
