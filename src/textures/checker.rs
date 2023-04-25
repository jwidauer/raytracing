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
            odd: Box::new(SolidColor::new(odd).into()),
            even: Box::new(SolidColor::new(even).into()),
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
