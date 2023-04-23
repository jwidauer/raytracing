mod checker;
mod noise;
mod solid_color;

pub use checker::Checker;
pub use noise::Noise;
pub use solid_color::SolidColor;

use dyn_clonable::clonable;

use crate::{color::Color, vec3::Vec3};

#[clonable]
pub trait Texture: Clone {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color;
}

pub type BoxedTexture<'a> = Box<dyn Texture + Send + Sync + 'a>;
