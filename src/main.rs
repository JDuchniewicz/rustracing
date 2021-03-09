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
use material::{Dielectric, Lambertian, Material, Metal};
use ray::Ray;
use sphere::Sphere;
use std::io::{self, Write};
use std::rc::Rc;
use utils::{random_f64, random_f64_minmax};
use vec3::{Color, Point3, Vec3};

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::with_values(&Color::with_values(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::with_values(
        Point3::with_values(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Point3::with_values(
                a as f64 + 0.9 * random_f64(),
                0.2,
                b as f64 + 0.9 * random_f64(),
            );

            if (center - Point3::with_values(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::vec3_random() * Vec3::vec3_random();
                    sphere_material = Rc::new(Lambertian::with_values(&albedo));
                    world.add(Rc::new(Sphere::with_values(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::vec3_random_minmax(0.5, 1.0);
                    let fuzz = random_f64_minmax(0.0, 0.5);
                    sphere_material = Rc::new(Metal::with_values(&albedo, fuzz));
                    world.add(Rc::new(Sphere::with_values(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Rc::new(Dielectric::with_values(1.5));
                    world.add(Rc::new(Sphere::with_values(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::with_values(1.5));
    world.add(Rc::new(Sphere::with_values(
        Point3::with_values(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::with_values(&Color::with_values(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::with_values(
        Point3::with_values(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::with_values(&Color::with_values(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::with_values(
        Point3::with_values(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

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
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;

    // World
    let world: HittableList = random_scene();

    // Camera
    let lookfrom = Point3::with_values(13.0, 2.0, 3.0);
    let lookat = Point3::with_values(0.0, 0.0, 0.0);
    let vup = Vec3::with_values(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam: Camera = Camera::new(
        &lookfrom,
        &lookat,
        &vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

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
            match write_color(&mut handle, pixel_color, SAMPLES_PER_PIXEL) {
                Ok(_) => continue,
                Err(e) => eprint!(
                    "Oops, error {} saving pixel {} for indices i {} j {}",
                    e, pixel_color, i, j
                ),
            }
        }
    }
    eprintln!("Done");
}
