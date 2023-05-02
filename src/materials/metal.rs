use crate::{color::Color, objects::HitRecord, ray::Ray};

use super::{diffusers, Material, ScatterRecord};

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Color,
    fuzziness: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f32) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = ray.direction().reflect(hit_record.normal)
            + self.fuzziness * diffusers::random_in_unit_sphere();
        let scattered = Ray::new_time_based(hit_record.point, reflected, ray.time());

        if scattered.direction().dot(hit_record.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}
