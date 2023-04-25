use crate::{aabb::AABB, materials::BoxedMaterial, ray::Ray, vec3::Vec3};

use super::{HitRecord, Object};

#[derive(Clone)]
pub struct Triangle<'a> {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    material: BoxedMaterial<'a>,
}

impl<'a> Triangle<'a> {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: BoxedMaterial<'a>) -> Self {
        Self {
            v0,
            v1,
            v2,
            material,
        }
    }
}

impl Object for Triangle<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Möller–Trumbore intersection algorithm
        // Reference: https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;

        let h = ray.direction().cross(&e2);
        let a = e1.dot(&h);
        if a.abs() < 1e-8 {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.v0;
        let u = f * s.dot(&h);
        if !(0.0..1.0).contains(&u) {
            return None;
        }

        let q = s.cross(&e1);
        let v = f * ray.direction().dot(&q);
        if !(0.0..(1.0 - u)).contains(&v) {
            return None;
        }

        let t = f * e2.dot(&q);
        if !(t_min..t_max).contains(&t) {
            return None;
        }

        let point = ray.at(t);
        let normal = e1.cross(&e2).normalized();
        let (normal, front_face) = HitRecord::orient_towards_ray(ray, normal);
        Some(super::HitRecord {
            point,
            normal,
            t,
            front_face,
            material: &*self.material,
            u,
            v,
        })
    }

    fn bounding_box(&self, _timeframe: crate::time::Time) -> Option<AABB> {
        let mut min = Vec3::new(
            self.v0.x().min(self.v1.x()).min(self.v2.x()),
            self.v0.y().min(self.v1.y()).min(self.v2.y()),
            self.v0.z().min(self.v1.z()).min(self.v2.z()),
        );
        let mut max = Vec3::new(
            self.v0.x().max(self.v1.x()).max(self.v2.x()),
            self.v0.y().max(self.v1.y()).max(self.v2.y()),
            self.v0.z().max(self.v1.z()).max(self.v2.z()),
        );
        // Make sure the bounding box is not degenerate
        if min.x() == max.x() || min.y() == max.y() || min.z() == max.z() {
            min -= Vec3::new(0.0001, 0.0001, 0.0001);
            max += Vec3::new(0.0001, 0.0001, 0.0001);
        }
        Some(AABB::new(min, max))
    }
}
