use crate::{color::Color, vec3::Vec3};

use super::{SolidColor, Texture, TextureEnum};

#[derive(Clone)]
pub struct Checker {
    odd: Box<TextureEnum>,
    even: Box<TextureEnum>,
}

impl Checker {
    pub fn from_colors(odd: Color, even: Color) -> Self {
        Self {
            odd: Box::new(TextureEnum::SolidColor(SolidColor::new(odd))),
            even: Box::new(TextureEnum::SolidColor(SolidColor::new(even))),
        }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color {
        let sines = (10.0 * point.x()).sin() * (10.0 * point.y()).sin() * (10.0 * point.z()).sin();

        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}
