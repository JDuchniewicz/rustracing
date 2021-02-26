mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use camera::Camera;
use color::write_color;
use hittable::Hittable;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
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

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new();
    }

    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit_record
            .material
            .as_ref()
            .unwrap()
            .scatter(ray, &hit_record)
        {
            let target: Point3 = hit_record.p + Vec3::random_in_hemisphere(&hit_record.normal);
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Color::new();
        }
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
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world: HittableList = HittableList::new();

    let material_ground = Rc::new(Lambertian::with_values(&Color::with_values(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::with_values(&Color::with_values(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::with_values(1.5));
    let material_right = Rc::new(Metal::with_values(&Color::with_values(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::with_values(
        Point3::with_values(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::with_values(
        Point3::with_values(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::with_values(
        Point3::with_values(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::with_values(
        Point3::with_values(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    )));
    world.add(Rc::new(Sphere::with_values(
        Point3::with_values(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let cam: Camera = Camera::new();

    // Render

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..=IMAGE_HEIGHT - 1).into_iter().rev() {
        eprintln!("Scanlines remaining {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new();
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_f64()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_f64()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }
            write_color(&mut handle, pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done");
}
