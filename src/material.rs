use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};
use rand::random;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn duplicate(&self) -> Box<dyn Material>;
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>
    where
        Self: Sized,
    {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector(&mut rand::thread_rng());

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }

    fn duplicate(&self) -> Box<dyn Material> {
        Box::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, f: f64) -> Metal {
        Metal {
            albedo,
            fuzz: f.clamp(-f64::INFINITY, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>
    where
        Self: Sized,
    {
        let reflected = ray_in.direction.normalize().reflect(rec.normal);

        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(&mut rand::thread_rng()),
        );
        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }

    fn duplicate(&self) -> Box<dyn Material> {
        Box::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>
    where
        Self: Sized,
    {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio;
        if rec.front_face {
            refraction_ratio = 1.0 / self.ir;
        } else {
            refraction_ratio = self.ir;
        }

        let unit_direction = ray_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;

        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random() {
            direction = unit_direction.reflect(rec.normal);
        } else {
            direction = unit_direction.refract(rec.normal, refraction_ratio);
        }

        let scattered = Ray::new(rec.p, direction);
        Some((attenuation, scattered))
    }

    fn duplicate(&self) -> Box<dyn Material> {
        Box::new(*self)
    }
}
