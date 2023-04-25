use crate::{aabb::AABB, ray::Ray, time::Time};

use super::{BoxedObject, HitRecord, Object};

#[derive(Clone)]
pub struct ObjectList<'a> {
    objects: Vec<BoxedObject<'a>>,
}

impl<'a> ObjectList<'a> {
    pub fn new(objects: Vec<BoxedObject<'a>>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: impl Object + 'a + Send + Sync) {
        self.objects.push(Box::new(object));
    }

    pub fn objects(&self) -> &[BoxedObject<'a>] {
        &self.objects
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

    fn bounding_box(&self, timeframe: Time) -> Option<crate::aabb::AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut output_box: Option<AABB> = None;

        for object in &self.objects {
            if let Some(tmp_box) = object.bounding_box(timeframe) {
                output_box = match output_box {
                    Some(output_box) => Some(AABB::surrounding_box(&output_box, &tmp_box)),
                    None => Some(tmp_box),
                };
            } else {
                return None;
            }
        }

        output_box
    }
}
