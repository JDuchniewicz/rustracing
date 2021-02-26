use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn from_hittable(object: Rc<dyn Hittable>) -> HittableList {
        let mut list = HittableList {
            objects: Vec::new(),
        };
        list.add(object);
        list
    }

    pub fn clear(&mut self) -> () {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) -> () {
        self.objects.push(object)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            match object.hit(ray, t_min, closest_so_far) {
                Some(rec) => {
                    closest_so_far = rec.t;
                    temp_rec.replace(rec);
                }
                None => {}
            }
        }

        temp_rec
    }
}
