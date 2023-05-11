use indicatif::ProgressBar;

use crate::camera::Camera;
use crate::image::Image;
use crate::scenes::Scene;
pub use crate::settings::Settings;

mod aabb;
mod camera;
mod color;
mod image;
mod materials;
mod objects;
mod perlin;
mod rand_ext;
mod ray;
mod renderer;
pub mod scenes;
pub mod settings;
mod textures;
mod time;
pub mod vec3;

pub trait ProgressTicker {
    fn tick(&self);
}

impl ProgressTicker for ProgressBar {
    fn tick(&self) {
        self.inc(1);
    }
}

fn setup_scene(settings: Settings) -> (Scene<'static>, Camera, Image) {
    // Camera
    let camera = Camera::from_settings(settings.camera);
    // World
    let world = Scene::from_settings(settings.scene);

    // Image
    let image = Image::from_settings(settings.image);

    (world, camera, image)
}

pub fn render_image_from_settings(
    settings: Settings,
    progress_ticker: &(impl ProgressTicker + Sync),
) -> Image {
    let (world, camera, mut image) = setup_scene(settings);

    // Renderer
    let renderer = renderer::Renderer::from_settings(settings.renderer);

    // Render
    renderer.render_image(&mut image, &world, &camera, progress_ticker);

    image
}
