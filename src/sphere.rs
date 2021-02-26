use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
};
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            center: Point3::new(),
            radius: 0.0,
            material: None,
        }
    }

    pub fn with_values(center: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material: Some(material),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec: Option<HitRecord> = Some(HitRecord::with_values(
            ray.at(root),
            root,
            self.material.as_ref().unwrap().clone(),
        ));
        let outward_normal: Vec3 = (rec.as_ref().unwrap().p - self.center) / self.radius;
        rec.as_mut().unwrap().set_face_normal(ray, &outward_normal);

        rec
    }
}
