use crate::{
    textures::{SolidColor, TextureEnum},
    vec3::Vec3,
};

use super::Material;

#[derive(Clone)]
pub struct DiffuseLight {
    emit: TextureEnum,
}

impl DiffuseLight {
    pub fn from_texture(emit: TextureEnum) -> Self {
        Self { emit }
    }

    pub fn from_color(color: crate::color::Color) -> Self {
        Self {
            emit: SolidColor::new(color).into(),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _ray: &crate::ray::Ray,
        _hit_record: &crate::objects::HitRecord,
    ) -> Option<super::ScatterRecord> {
        None
    }

    fn emitted(&self, u: f64, v: f64, point: &Vec3) -> crate::color::Color {
        self.emit.value(u, v, point)
    }
}
