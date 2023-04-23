use crate::{
    color::Color,
    objects::HitRecord,
    ray::Ray,
    textures::{BoxedTexture, SolidColor, Texture},
};

use super::{diffusers, Material, ScatterRecord};

#[derive(Clone)]
pub struct Lambertian<'a> {
    texture: BoxedTexture<'a>,
}

impl<'a> Lambertian<'a> {
    pub fn new(albedo: Color) -> Self {
        Self {
            texture: Box::new(SolidColor::new(albedo)),
        }
    }

    pub fn from_texture(texture: impl Texture + Send + Sync + 'a) -> Self {
        Self {
            texture: Box::new(texture),
        }
    }
}

impl Material for Lambertian<'_> {
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
