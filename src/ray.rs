use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            origin: Point3::new(),
            direction: Vec3::new(),
        }
    }

    pub fn with_values(orig: Point3, dir: Vec3) -> Ray {
        Ray {
            origin: orig,
            direction: dir,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        Point3::with_vec3(self.origin + t * self.direction)
    }
}
