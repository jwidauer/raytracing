use crate::{
    color::Color,
    objects::HitRecord,
    ray::Ray,
    textures::{BoxedTexture, SolidColor, Texture},
    vec3::Vec3,
};

use super::{Material, ScatterRecord};

#[derive(Clone)]
pub struct Isotropic<'a> {
    albedo: BoxedTexture<'a>,
}

impl<'a> Isotropic<'a> {
    pub fn new(albedo: impl Texture + 'a + Sync + Send) -> Self {
        Self::new_boxed(Box::new(albedo))
    }

    pub fn new_boxed(albedo: BoxedTexture<'a>) -> Self {
        Self { albedo }
    }

    pub fn from_color(color: Color) -> Self {
        Self::new(SolidColor::new(color))
    }
}

impl Material for Isotropic<'_> {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, &hit_record.point);
        let scattered =
            Ray::new_time_based(hit_record.point, Vec3::random_in_unit_sphere(), ray.time());

        Some(ScatterRecord {
            attenuation,
            scattered,
        })
    }
}
