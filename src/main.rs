use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use prettytable::table;
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
mod perlin;
mod rand_ext;
mod ray;
mod scenes;
mod textures;
mod time;
pub mod vec3;

fn setup_scene(scene_type: SceneType, time: Time) -> (Scene<'static>, Camera, Image) {
    // Camera
    let mut cam_pos = Point3::new(13.0, 2.0, 3.0);
    let mut look_at = Point3::new(0.0, 0.0, 0.0);
    let mut vfov = 20.0;
    let mut aperture = 0.0;
    let mut background = Color::new(0.7, 0.8, 1.0);
    let mut aspect_ratio = 16.0 / 9.0;
    let image_height = 720;

    match scene_type {
        SceneType::BookCover => {
            aperture = 0.1;
        }
        SceneType::SimpleLight => {
            cam_pos = Point3::new(0.0, 3.0, 26.0);
            look_at = Point3::new(0.0, 2.0, 0.0);
            background = Color::new(0.0, 0.0, 0.0);
        }
        SceneType::CornellBox => {
            cam_pos = Point3::new(278.0, 278.0, -800.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            background = Color::new(0.0, 0.0, 0.0);
            vfov = 40.0;
            aspect_ratio = 1.0;
        }
        SceneType::CornellSmoke => {
            cam_pos = Point3::new(278.0, 278.0, -800.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            background = Color::new(0.0, 0.0, 0.0);
            vfov = 40.0;
            aspect_ratio = 1.0;
        }
        SceneType::FinalScene => {
            cam_pos = Point3::new(478.0, 278.0, -600.0);
            look_at = Point3::new(278.0, 278.0, 0.0);
            background = Color::new(0.0, 0.0, 0.0);
            vfov = 40.0;
            aspect_ratio = 1.0;
        }
        _ => {}
    }

    let vup = Point3::new(0.0, 1.0, 0.0);
    let focus_dist = 10.0;

    // Camera
    let camera = Camera::new(
        Ray::new(cam_pos, cam_pos - look_at),
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        time,
    );

    // World
    let world = Scene::new(scene_type, time, background);

    // Image
    let width = (image_height as f32 * aspect_ratio) as usize;
    let image = Image::new(width, image_height);

    (world, camera, image)
}

fn seperated<T>(num: T) -> String
where
    T: std::fmt::Display,
{
    num.to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(",")
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// The scene to render
    #[arg(short = 't', long, value_enum)]
    scene_type: SceneType,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let samples_per_pixel = 500;
    let max_depth = 50;

    let timeframe = Time::from_exposure(1.0);

    let (world, camera, image) = setup_scene(args.scene_type, timeframe);
    let mut image = image;

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
                let (r_u, r_v) = rand::random::<(f32, f32)>();
                let u = (x as f32 + r_u) / (image.width - 1) as f32;
                let v = (y as f32 + r_v) / (image.height - 1) as f32;

                let ray = camera.get_ray(u, v);

                pixel_color += world.ray_color(&ray, max_depth);
            }

            *pixel = pixel_color / samples_per_pixel as f32;

            progress.inc(1);
        });

    let render_time = now.elapsed();
    let single_core_render_time = render_time.mul_f32(num_cpus::get() as f32);

    image.write_ppm("img.ppm")?;
    progress.finish();

    println!("Done!");
    let table = table!(
        ["Nr. of threads", num_cpus::get()],
        [
            "Total render time",
            format!("{}ms", seperated(render_time.as_millis()))
        ],
        [
            "Total single core render time",
            format!("{:.2}s", single_core_render_time.as_secs_f32())
        ],
        [
            "Time per sample",
            format!(
                "{:.2}ns",
                render_time.as_nanos() as f32
                    / (image.width * image.height * samples_per_pixel) as f32
            )
        ],
        [
            "Single core time per sample",
            format!(
                "{:.2}ns",
                single_core_render_time.as_nanos() as f32
                    / (image.width * image.height * samples_per_pixel) as f32
            )
        ]
    );
    table.printstd();

    Ok(())
}
