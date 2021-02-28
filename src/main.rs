mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::write_color;
use hittable::Hittable;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use rand::Rng;
use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;
use std::{
    io::{stderr, Write},
    sync::Arc,
};
use vec3::{Color, Point3, Vec3};

fn random_scene(rng: &mut impl rand::Rng) -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::vec3_random(rng) * Vec3::vec3_random(rng);

                    world.add(Sphere::new(center, 0.2, Lambertian::new(albedo)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::vec3_random_range(rng, 0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);

                    world.add(Sphere::new(center, 0.2, Metal::new(albedo, fuzz)));
                } else {
                    // glass

                    world.add(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    world
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(ray: &Ray, world: &impl Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit_record
            .material
            .as_ref()
            .unwrap()
            .scatter(&ray, &hit_record)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Color::default();
        }
    }
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // TODO: error handling
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();

    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const NUM_PIXELS: i32 = IMAGE_WIDTH * IMAGE_HEIGHT;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 500;

    // Cache thread rng
    let mut rng = rand::thread_rng();

    // World
    let world = Arc::new(random_scene(&mut rng));

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    // Render

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let n_finished = Arc::new(std::sync::atomic::AtomicI32::new(0));

    let pixels = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .map(move |j| {
            let world = world.clone();
            let n_finished = n_finished.clone();
            (0..IMAGE_WIDTH).into_par_iter().map(move |i| {
                let world = world.clone();
                let n_finished = n_finished.clone();

                let mut rng = rand::thread_rng();

                let mut pixel_color = Color::default();
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                    let ray = cam.get_ray(u, v);
                    pixel_color += ray_color(&ray, world.as_ref(), MAX_DEPTH);
                }

                let n = n_finished.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                if n % (NUM_PIXELS / 1000) == 0 || n == NUM_PIXELS - 1 {
                    eprint!(
                        "\rCalculated {}/{} pixels ({:.1?}%)",
                        n + 1,
                        NUM_PIXELS,
                        (n + 1) as f64 / NUM_PIXELS as f64 * 100.0,
                    );
                    stderr().flush().unwrap();
                }

                pixel_color
            })
        })
        .flatten();

    let mut pixel_vec = Vec::with_capacity(NUM_PIXELS as usize);
    pixel_vec.par_extend(pixels);

    eprintln!();

    for (i, pixel_color) in pixel_vec.into_iter().enumerate() {
        if i as i32 % (NUM_PIXELS / 1000) == 0 || i as i32 == NUM_PIXELS - 1 {
            let n = i + 1;
            eprint!(
                "\rWriting pixel {}/{} ({:.1?}%)",
                n,
                NUM_PIXELS,
                n as f64 / NUM_PIXELS as f64 * 100.0,
            );

            stderr().flush().unwrap();
        }

        write_color(&mut handle, pixel_color, SAMPLES_PER_PIXEL).unwrap_or_else(|err| {
            panic!(
                "Oops, error {} saving color {} for pixel {}/{}",
                err,
                pixel_color,
                i + 1,
                NUM_PIXELS
            )
        })
    }

    eprintln!();

    eprintln!("Done");
}
