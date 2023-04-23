use crate::{materials::Material, ray::Ray, vec3::Point3};

use super::{HitRecord, Object};

pub struct Sphere<'a> {
    center: Point3,
    radius: f64,
    material: Box<dyn Material + 'a + Send + Sync>,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f64, material: impl Material + 'a + Send + Sync) -> Self {
        Self::new_boxed(center, radius, Box::new(material))
    }

    pub fn new_boxed(
        center: Point3,
        radius: f64,
        material: Box<dyn Material + 'a + Send + Sync>,
    ) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Object for Sphere<'_> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
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
            let normal = (p - self.center) / self.radius;
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
}
