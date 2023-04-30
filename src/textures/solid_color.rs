use crate::{color::Color, vec3::Vec3};

use super::Texture;

#[derive(Debug, Clone)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _point: &Vec3) -> Color {
        self.color
    }
}
