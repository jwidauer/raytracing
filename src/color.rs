extern crate overload;
use overload::overload;
use std::ops;

use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(Vec3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Vec3::new(r, g, b))
    }

    pub fn r(&self) -> f32 {
        self.0[0]
    }

    pub fn g(&self) -> f32 {
        self.0[1]
    }

    pub fn b(&self) -> f32 {
        self.0[2]
    }

    pub fn random() -> Self {
        Self(Vec3::random())
    }

    pub fn random_range(min: f32, max: f32) -> Self {
        Self(Vec3::random_range(min, max))
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Gamma correction
        let r = self.0[0].sqrt();
        let g = self.0[1].sqrt();
        let b = self.0[2].sqrt();

        let ir = (256.0 * r.clamp(0.0, 0.999)) as u8;
        let ig = (256.0 * g.clamp(0.0, 0.999)) as u8;
        let ib = (256.0 * b.clamp(0.0, 0.999)) as u8;

        write!(f, "{} {} {}", ir, ig, ib)
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Self(v)
    }
}

overload!((a: ?Color) + (b: ?Color) -> Color {Color(a.0 + b.0)});
overload!((a: ?Color) / (b: f32) -> Color {Color(a.0 / b)});
overload!((a: ?Color) * (b: ?Color) -> Color {Color(a.0 * b.0)});
overload!((a: f32) * (b: ?Color) -> Color {Color(a * b.0)});
overload!((a: ?Color) * (b: f32) -> Color {b * a});

// Assignment operators
overload!((a: &mut Color) += (b: ?Color) {a.0 += b.0});
overload!((a: &mut Color) *= (b: ?Color) {a.0 *= b.0});
