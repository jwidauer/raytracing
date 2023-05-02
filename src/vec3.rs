use std::ops::{self, Index};
extern crate overload;
use overload::overload;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    e: [f32; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0., 0., 0.)
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn normalized(&self) -> Self {
        self / self.length()
    }

    pub fn dot(&self, v: Vec3) -> f32 {
        self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
    }

    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * v.e[2] - self.e[2] * v.e[1],
            self.e[2] * v.e[0] - self.e[0] * v.e[2],
            self.e[0] * v.e[1] - self.e[1] * v.e[0],
        )
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(&self, normal: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (-self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }

    pub fn rotate(&self, axis: Vec3, angle: f32) -> Vec3 {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        cos_theta * self + sin_theta * self.cross(axis) + (1.0 - cos_theta) * axis * self.dot(axis)
    }

    pub fn near_zero(&self) -> bool {
        let epsilon = 1e-8;
        self.e[0].abs() < epsilon && self.e[1].abs() < epsilon && self.e[2].abs() < epsilon
    }

    pub fn random() -> Vec3 {
        let (x, y, z): (f32, f32, f32) = rand::random();
        Vec3::new(x, y, z)
    }

    pub fn random_range(min: f32, max: f32) -> Vec3 {
        let (x, y, z): (f32, f32, f32) = rand::random();
        Vec3::new(
            x * (max - min) + min,
            y * (max - min) + min,
            z * (max - min) + min,
        )
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

// Binary operators
overload!((a: ?Vec3) + (b: ?Vec3) -> Vec3 { Vec3::new(a.e[0] + b.e[0], a.e[1] + b.e[1], a.e[2] + b.e[2]) });
overload!((a: ?Vec3) - (b: ?Vec3) -> Vec3 { Vec3::new(a.e[0] - b.e[0], a.e[1] - b.e[1], a.e[2] - b.e[2]) });
overload!((a: ?Vec3) * (b: ?Vec3) -> Vec3 { Vec3::new(a.e[0] * b.e[0], a.e[1] * b.e[1], a.e[2] * b.e[2]) });
overload!((a: ?Vec3) * (b: f32) -> Vec3 { Vec3::new(a.e[0] * b, a.e[1] * b, a.e[2] * b) });
overload!((a: f32) * (b: ?Vec3) -> Vec3 { b * a });
overload!((a: ?Vec3) / (b: f32) -> Vec3 { Vec3::new(a.e[0] / b, a.e[1] / b, a.e[2] / b) });

// Assignment operators
overload!((a: &mut Vec3) += (b: ?Vec3) { a.e[0] += b.e[0]; a.e[1] += b.e[1]; a.e[2] += b.e[2]; });
overload!((a: &mut Vec3) -= (b: ?Vec3) { a.e[0] -= b.e[0]; a.e[1] -= b.e[1]; a.e[2] -= b.e[2]; });
overload!((a: &mut Vec3) *= (b: ?Vec3) { a.e[0] *= b.e[0]; a.e[1] *= b.e[1]; a.e[2] *= b.e[2]; });
overload!((a: &mut Vec3) *= (b: f32) { a.e[0] *= b; a.e[1] *= b; a.e[2] *= b; });
overload!((a: &mut Vec3) /= (b: f32) { a.e[0] /= b; a.e[1] /= b; a.e[2] /= b; });

// Unary operators
overload!(- (a: ?Vec3) -> Vec3 { Vec3::new(-a.e[0], -a.e[1], -a.e[2]) });

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
