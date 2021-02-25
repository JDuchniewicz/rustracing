mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use color::write_color;
use hittable::Hittable;
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use std::io::{self, Write};
use std::rc::Rc;
use vec3::{Color, Point3, Vec3};

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = Vec3::dot(&oc, &ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    if let Some(hit_record) = world.hit(ray, 0.0, f64::INFINITY) {
        return 0.5 * (hit_record.normal + Color::with_values(1.0, 1.0, 1.0));
    }
    let unit_direction: Vec3 = Vec3::unit_vector(ray.direction);
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

    // World
    let mut world: HittableList = HittableList::new();
    world.add(Rc::new(Box::new(Sphere::with_values(
        Point3::with_values(0.0, 0.0, -1.0),
        0.5,
    ))));
    world.add(Rc::new(Box::new(Sphere::with_values(
        Point3::with_values(0.0, -100.5, -1.0),
        100.0,
    ))));

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
            let pixel_color: Color = ray_color(&r, &world);
            write_color(&mut handle, pixel_color);
        }
    }
    eprintln!("Done");
}
