use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::rc::Rc;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Option<Rc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, material: Option<Rc<dyn Material>>) -> HitRecord {
        HitRecord {
            p,
            normal: p,
            material,
            t,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
