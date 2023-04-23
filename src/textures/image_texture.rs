use crate::color::Color;

use super::Texture;

#[derive(Clone)]
pub struct ImageTexture {
    image: image::RgbImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let image = image::open(filename).unwrap().to_rgb8();
        Self { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _point: &crate::vec3::Vec3) -> crate::color::Color {
        let u = u.clamp(0., 1.);
        let v = v.clamp(0., 1.);

        let i = ((u * self.image.width() as f64) as u32).min(self.image.width() - 1);
        let j =
            (((1. - v) * self.image.height() as f64 - 0.001) as u32).min(self.image.height() - 1);

        let pixel = self.image.get_pixel(i, j);
        Color::new(
            pixel[0] as f64 / 255.,
            pixel[1] as f64 / 255.,
            pixel[2] as f64 / 255.,
        )
    }
}
