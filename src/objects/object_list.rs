use crate::ray::Ray;

use super::{HitRecord, Object};

pub struct ObjectList<'a> {
    objects: Vec<Box<dyn Object + 'a + Send + Sync>>,
}

impl<'a> ObjectList<'a> {
    pub fn new(objects: Vec<Box<dyn Object + Send + Sync>>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: impl Object + 'a + Send + Sync) {
        self.objects.push(Box::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Object for ObjectList<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}
