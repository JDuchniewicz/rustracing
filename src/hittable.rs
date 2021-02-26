use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn with_values(p: Point3, t: f64) -> HitRecord {
        HitRecord {
            p,
            normal: p,
            t,
            front_face: false,
        }
    }

    #[inline]
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) -> () {
        self.front_face = Vec3::dot(&ray.direction, outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = -outward_normal.clone();
        }
    }
}