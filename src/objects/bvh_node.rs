use std::{cmp::Ordering, sync::Arc};

use crate::{aabb::AABB, time::Time};

use super::{BoxedObject, Object, ObjectList};

#[derive(Clone)]
pub struct BvhNode<'a> {
    left: BoxedObject<'a>,
    right: BoxedObject<'a>,
    bounding_box: AABB,
}

impl<'a> BvhNode<'a> {
    pub fn new(objects: &[BoxedObject<'a>], timeframe: Time) -> Self {
        let axis: usize = (3.0 * rand::random::<f64>()) as usize;
        let comparator = |a: &_, b: &_| BvhNode::compare_along_axis(a, b, axis);

        let left;
        let right;

        if objects.len() == 1 {
            // Copy the object into the left and right nodes
            left = objects[0].clone();
            right = objects[0].clone();
        } else if objects.len() == 2 {
            match comparator(&objects[0], &objects[1]) {
                Ordering::Less | Ordering::Equal => {
                    left = objects[0].clone();
                    right = objects[1].clone();
                }
                Ordering::Greater => {
                    left = objects[1].clone();
                    right = objects[0].clone();
                }
            }
        } else {
            let mut objects = objects.to_vec();
            objects.sort_by(comparator);

            let mid = objects.len() / 2;
            left = Arc::new(BvhNode::new(&objects[..mid], timeframe));
            right = Arc::new(BvhNode::new(&objects[mid..], timeframe));
        }

        let box_left = left.bounding_box(timeframe).unwrap();
        let box_right = right.bounding_box(timeframe).unwrap();

        Self {
            left,
            right,
            bounding_box: AABB::surrounding_box(&box_left, &box_right),
        }
    }

    pub fn from_list(list: &ObjectList<'a>, timeframe: Time) -> Self {
        Self::new(&list.objects(), timeframe)
    }

    fn compare_along_axis<'b>(a: &BoxedObject<'b>, b: &BoxedObject<'b>, axis: usize) -> Ordering {
        if axis > 2 {
            panic!("Invalid axis");
        }

        let box_a = a.bounding_box(Time::new(0., 0.)).unwrap();
        let box_b = b.bounding_box(Time::new(0., 0.)).unwrap();

        box_a.min()[axis].partial_cmp(&box_b.min()[axis]).unwrap()
    }
}

impl Object for BvhNode<'_> {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<super::HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right = self.right.hit(ray, t_min, t_max);

        match (hit_left, hit_right) {
            (Some(left), Some(right)) => {
                if left.t < right.t {
                    Some(left)
                } else {
                    Some(right)
                }
            }
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (None, None) => None,
        }
    }

    fn bounding_box(&self, _timeframe: Time) -> Option<AABB> {
        Some(self.bounding_box)
    }
}
