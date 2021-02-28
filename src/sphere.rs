use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
};

#[derive(Default)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Option<Box<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: impl Material + 'static) -> Self {
        Sphere {
            center,
            radius,
            material: Some(Box::new(material)),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = -(sqrtd + half_b) / a;
        if root < t_min || root > t_max {
            root = (sqrtd - half_b) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let mut rec = HitRecord::new(ray.at(root), root, self.material.as_ref().map(Box::as_ref));

        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        Some(rec)
    }
}
