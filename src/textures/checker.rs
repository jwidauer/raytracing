use crate::{color::Color, vec3::Vec3};

use super::{BoxedTexture, SolidColor, Texture};

#[derive(Clone)]
pub struct Checker<'a> {
    odd: BoxedTexture<'a>,
    even: BoxedTexture<'a>,
}

impl<'a> Checker<'a> {
    pub fn from_colors(odd: Color, even: Color) -> Self {
        Self {
            odd: Box::new(SolidColor::new(odd)),
            even: Box::new(SolidColor::new(even)),
        }
    }
}

impl Texture for Checker<'_> {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color {
        let sines = (10.0 * point.x()).sin() * (10.0 * point.y()).sin() * (10.0 * point.z()).sin();

        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}
