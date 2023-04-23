use crate::{color::Color, perlin::Perlin};

use super::Texture;

#[derive(Clone)]
pub struct Noise {
    noise: Perlin,
    scale: f64,
}

impl Noise {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f64, _v: f64, point: &crate::vec3::Vec3) -> crate::color::Color {
        Color::new(1., 1., 1.) * 0.5 * (1. + self.noise.noise(&(point * self.scale)))
    }
}
