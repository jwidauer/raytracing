use crate::{
    textures::{self, BoxedTexture, Texture},
    vec3::Vec3,
};

use super::Material;

#[derive(Clone)]
pub struct DiffuseLight<'a> {
    emit: BoxedTexture<'a>,
}

impl<'a> DiffuseLight<'a> {
    pub fn from_texture(emit: impl Texture + Send + Sync + 'a) -> Self {
        Self {
            emit: Box::new(emit),
        }
    }

    pub fn from_color(color: crate::color::Color) -> Self {
        Self {
            emit: Box::new(textures::SolidColor::new(color)),
        }
    }
}

impl Material for DiffuseLight<'_> {
    fn scatter(
        &self,
        _ray: &crate::ray::Ray,
        _hit_record: &crate::objects::HitRecord,
    ) -> Option<super::ScatterRecord> {
        None
    }

    fn emitted(&self, u: f32, v: f32, point: &Vec3) -> crate::color::Color {
        self.emit.value(u, v, point)
    }
}
