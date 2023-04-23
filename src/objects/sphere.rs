use crate::{
    aabb::AABB,
    materials::{BoxedMaterial, Material},
    ray::Ray,
    vec3::{Point3, Vec3},
};

use super::{HitRecord, Object};

#[derive(Clone)]
pub struct Sphere<'a> {
    movement: Ray,
    radius: f64,
    material: BoxedMaterial<'a>,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, material: impl Material + 'a + Send + Sync) -> Self {
        Self::new_boxed(center, radius, Box::new(material))
    }

    pub fn new_boxed(center: Point3, radius: f64, material: BoxedMaterial<'a>) -> Self {
        Self {
            movement: Ray::new(center, Vec3::zero()),
            radius,
            material,
        }
    }

    pub fn new_moving(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: impl Material + 'a + Send + Sync,
    ) -> Self {
        Self::new_moving_boxed(center0, center1, time0, time1, radius, Box::new(material))
    }

    pub fn new_moving_boxed(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: BoxedMaterial<'a>,
    ) -> Self {
        let movement = Ray::new(center0, (center1 - center0) / (time1 - time0));
        Self {
            movement,
            radius,
            material,
        }
    }
}

impl Object for Sphere<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.movement.at(ray.time());
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_disc = discriminant.sqrt();

        let calc_hit_record = |root: f64| {
            let p = ray.at(root);
            let normal = (p - self.movement.at(ray.time())) / self.radius;
            let (normal, front_face) = HitRecord::orient_towards_ray(ray, normal);
            Some(HitRecord {
                point: p,
                normal,
                t: root,
                front_face,
                material: self.material.as_ref(),
            })
        };

        let root = (-half_b - sqrt_disc) / a;
        if (t_min..t_max).contains(&root) {
            return calc_hit_record(root);
        }

        let root = (-half_b + sqrt_disc) / a;
        if (t_min..t_max).contains(&root) {
            return calc_hit_record(root);
        }

        None
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::aabb::AABB> {
        Some(AABB::new(
            self.movement.at(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.movement.at(time1) + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}
