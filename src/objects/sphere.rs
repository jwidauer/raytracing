use std::sync::Arc;

use crate::{
    aabb::AABB,
    materials::{BoxedMaterial, Material},
    ray::Ray,
    time::Time,
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
        Self::new_boxed(center, radius, Arc::new(material))
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
        timeframe: Time,
        radius: f64,
        material: impl Material + 'a + Send + Sync,
    ) -> Self {
        Self::new_moving_boxed(center0, center1, timeframe, radius, Arc::new(material))
    }

    pub fn new_moving_boxed(
        center0: Point3,
        center1: Point3,
        timeframe: Time,
        radius: f64,
        material: BoxedMaterial<'a>,
    ) -> Self {
        let movement = Ray::new(
            center0,
            (center1 - center0) / (timeframe.end - timeframe.start),
        );
        Self {
            movement,
            radius,
            material,
        }
    }

    // Compute the UV coordinates of a point on the surface of a unit sphere.
    pub fn sphere_uv(p: &Vec3) -> (f64, f64) {
        const PI: f64 = std::f64::consts::PI;

        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;
        (phi / (2.0 * PI), theta / PI)
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
            let (u, v) = Self::sphere_uv(&normal);
            Some(HitRecord {
                point: p,
                normal,
                t: root,
                front_face,
                material: self.material.as_ref(),
                u,
                v,
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

    fn bounding_box(&self, timeframe: Time) -> Option<crate::aabb::AABB> {
        Some(AABB::new(
            self.movement.at(timeframe.start) - Vec3::new(self.radius, self.radius, self.radius),
            self.movement.at(timeframe.end) + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}
