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

#[derive(Clone)]
pub enum TextureEnum {
    Checker(Checker),
    ImageTexture(ImageTexture),
    Noise(Noise),
    SolidColor(SolidColor),
}

impl TextureEnum {
    pub fn value(&self, u: f64, v: f64, point: &Vec3) -> Color {
        match self {
            TextureEnum::Checker(checker) => checker.value(u, v, point),
            TextureEnum::ImageTexture(image_texture) => image_texture.value(u, v, point),
            TextureEnum::Noise(noise) => noise.value(u, v, point),
            TextureEnum::SolidColor(solid_color) => solid_color.value(u, v, point),
        }
    }
}

impl From<Checker> for TextureEnum {
    fn from(checker: Checker) -> Self {
        Self::Checker(checker)
    }
}

impl From<ImageTexture> for TextureEnum {
    fn from(image_texture: ImageTexture) -> Self {
        Self::ImageTexture(image_texture)
    }
}

impl From<Noise> for TextureEnum {
    fn from(noise: Noise) -> Self {
        Self::Noise(noise)
    }
}

impl From<SolidColor> for TextureEnum {
    fn from(solid_color: SolidColor) -> Self {
        Self::SolidColor(solid_color)
    }
}

#[clonable]
pub trait Texture: Clone {
    fn value(&self, u: f64, v: f64, point: &Vec3) -> Color;
}

pub type BoxedTexture<'a> = Box<dyn Texture + Send + Sync + 'a>;
