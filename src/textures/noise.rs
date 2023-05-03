use crate::{color::Color, perlin::Perlin};

use super::Texture;

#[derive(Clone)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, point: &crate::vec3::Vec3) -> crate::color::Color {
        Color::new(1., 1., 1.)
            * 0.5
            * (1. + (self.scale * point.z() + 10. * self.noise.turb(point, 7)).sin())
    }
}
