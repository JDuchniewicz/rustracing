use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn from_hittable(object: impl Hittable + Sync + Send + 'static) -> HittableList {
        let mut list = HittableList {
            objects: Vec::new(),
        };
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: impl Hittable + Sync + Send + 'static) {
        self.objects.push(Box::new(object))
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec.replace(rec);
            }
        }

        temp_rec
    }
}
