use rayon::prelude::*;

use crate::{
    camera::Camera, color::Color, image::Image, scenes::Scene, settings::RendererSettings,
    ProgressTicker,
};

pub struct Renderer {
    samples_per_pixel: usize,
    max_depth: usize,
}

impl Renderer {
    pub fn new(samples_per_pixel: usize, max_depth: usize) -> Self {
        Self {
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn from_settings(settings: RendererSettings) -> Self {
        Self::new(settings.samples_per_pixel, settings.max_depth)
    }

    pub fn render_image(
        &self,
        image: &mut Image,
        world: &Scene,
        camera: &Camera,
        progress_ticker: &(impl ProgressTicker + Sync),
    ) {
        image
            .pixels
            .par_iter_mut()
            .enumerate()
            .for_each(|(idx, pixel)| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                let x = idx % image.width;
                let y = idx / image.width;

                for _ in 0..self.samples_per_pixel {
                    let (r_u, r_v) = rand::random::<(f32, f32)>();
                    let u = (x as f32 + r_u) / (image.width - 1) as f32;
                    let v = (y as f32 + r_v) / (image.height - 1) as f32;

                    let ray = camera.get_ray(u, v);

                    pixel_color += world.ray_color(&ray, self.max_depth);
                }

                *pixel = pixel_color / self.samples_per_pixel as f32;

                progress_ticker.tick();
            });
    }
}
