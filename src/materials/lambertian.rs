use crate::{color::Color, objects::HitRecord, ray::Ray};

use super::{diffusers, Material, ScatterRecord};

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction =
            hit_record.normal + diffusers::random_lambertian(&hit_record.normal);

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new_time_based(hit_record.point, scatter_direction, ray.time());

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered,
        })
    }
}
