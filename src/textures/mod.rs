mod checker;
mod image_texture;
mod noise;
mod solid_color;

pub use checker::Checker;
pub use image_texture::ImageTexture;
pub use noise::Noise;
pub use solid_color::SolidColor;

use dyn_clonable::clonable;

use crate::{color::Color, vec3::Vec3};

#[clonable]
pub trait Texture: Clone {
    fn value(&self, u: f32, v: f32, point: &Vec3) -> Color;
}

pub type BoxedTexture<'a> = Box<dyn Texture + Send + Sync + 'a>;
