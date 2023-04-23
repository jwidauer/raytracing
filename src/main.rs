use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use color::Color;
use ray::Ray;
use scenes::{Scene, SceneType};

use crate::{camera::Camera, image::Image, time::Time, vec3::Point3};

mod aabb;
mod camera;
mod color;
mod image;
mod materials;
mod objects;
mod rand_ext;
mod ray;
mod scenes;
mod textures;
mod time;
mod vec3;

fn setup_scene(aspect_ratio: f64, scene_type: SceneType, time: Time) -> (Scene<'static>, Camera) {
    let world = Scene::new(scene_type, time);

    // Camera
    let cam_pos;
    let look_at;
    let vfov;
    let aperture;

    match scene_type {
        SceneType::TwoSpheres => {
            cam_pos = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
        }
        SceneType::ThreeSpheres => {
            cam_pos = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.0;
        }
        SceneType::BookCover => {
            cam_pos = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
    }

    let vup = Point3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;

    let camera = Camera::new(
        Ray::new(cam_pos, cam_pos - look_at),
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        time,
    );

    (world, camera)
}

fn main() -> Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let height = 720;
    let width = (height as f64 * aspect_ratio) as usize;
    let mut image = Image::new(width, height);

    let samples_per_pixel = 100;
    let max_depth = 50;

    let timeframe = Time::from_exposure(1.0);

    let (world, camera) = setup_scene(aspect_ratio, SceneType::TwoSpheres, timeframe);

    // Set up progress bar
    let progress = ProgressBar::new((image.height * image.width) as u64).with_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40} {percent}%").unwrap(),
    );

    println!("Rendering {}x{} image...", image.width, image.height);

    let now = std::time::Instant::now();

    // Render
    image
        .pixels
        .par_iter_mut()
        .enumerate()
        .for_each(|(idx, pixel)| {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            let x = idx % image.width;
            let y = idx / image.width;

            for _ in 0..samples_per_pixel {
                let (r_u, r_v) = rand::random::<(f64, f64)>();
                let u = (x as f64 + r_u) / (image.width - 1) as f64;
                let v = (y as f64 + r_v) / (image.height - 1) as f64;

                let ray = camera.get_ray(u, v);

                pixel_color += world.ray_color(&ray, max_depth);
            }

            *pixel = pixel_color / samples_per_pixel as f64;

            progress.inc(1);
        });

    image.write_ppm("img.ppm")?;
    progress.finish();

    println!("Done! Took {}ms", now.elapsed().as_millis());

    Ok(())
}
