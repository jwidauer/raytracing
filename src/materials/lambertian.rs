use crate::{
    color::Color,
    objects::HitRecord,
    ray::Ray,
    textures::{SolidColor, TextureEnum},
};

use super::{diffusers, Material, ScatterRecord};

#[derive(Clone)]
pub struct Lambertian {
    texture: TextureEnum,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            texture: TextureEnum::SolidColor(SolidColor::new(albedo)),
        }
    }

    pub fn from_texture(texture: TextureEnum) -> Self {
        Self { texture: texture }
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
            attenuation: self
                .texture
                .value(hit_record.u, hit_record.v, &hit_record.point),
            scattered,
        })
    }
}
