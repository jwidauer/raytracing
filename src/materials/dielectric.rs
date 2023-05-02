use crate::{color::Color, objects::HitRecord, ray::Ray};

use super::{Material, ScatterRecord};

#[derive(Debug, Clone)]
pub struct Dielectric {
    attenuation: Color,
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self {
            attenuation: Color::new(1.0, 1.0, 1.0),
            refraction_index,
        }
    }

    pub fn from_color(color: Color, refraction_index: f32) -> Self {
        Self {
            attenuation: color,
            refraction_index,
        }
    }

    fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let etai_over_etat = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction().normalized();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // Check for total internal reflection
        let cannot_refract = etai_over_etat * sin_theta > 1.0;
        let reflactance_too_low = Self::reflectance(cos_theta, etai_over_etat) > rand::random();

        let direction = if cannot_refract || reflactance_too_low {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, etai_over_etat)
        };

        let scattered = Ray::new_time_based(rec.point, direction, ray.time());

        Some(ScatterRecord {
            attenuation: self.attenuation,
            scattered,
        })
    }
}
