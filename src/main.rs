mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod utils;
mod vec3;

use camera::Camera;
use color::write_color;
use hittable::Hittable;
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use std::io::{self, Write};
use std::rc::Rc;
use utils::random_f64;
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
    let samples_per_pixel = 100;

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
    let cam: Camera = Camera::new();

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..=image_height - 1).into_iter().rev() {
        eprintln!("Scanlines remaining {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new();
            for s in 0..samples_per_pixel {
                let u = (i as f64 + random_f64()) / (image_width - 1) as f64;
                let v = (j as f64 + random_f64()) / (image_height - 1) as f64;
                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }
            write_color(&mut handle, pixel_color, samples_per_pixel);
        }
    }
    eprintln!("Done");
}
