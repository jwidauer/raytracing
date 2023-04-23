use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use color::Color;
use objects::Object;
use ray::Ray;

use crate::{
    camera::Camera,
    image::Image,
    materials::{Dielectric, Lambertian, Metal},
    objects::Sphere,
    vec3::Point3,
};

mod camera;
mod color;
mod image;
mod materials;
mod objects;
mod ray;
mod vec3;

fn ray_color(r: &Ray, world: &impl Object, max_depth: usize) -> Color {
    coz::scope!("ray_color");
    let mut color = Color::new(1.0, 1.0, 1.0);
    let mut r = r.clone();
    for _ in 0..max_depth {
        if let Some(rec) = world.hit(&r, 0.001, f64::INFINITY) {
            if let Some(scatter) = rec.material.scatter(&r, &rec) {
                color *= scatter.attenuation;
                r = scatter.scattered;
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }
        } else {
            // Background
            let unit_direction = r.direction().normalized();
            let t = 0.5 * (unit_direction.y() + 1.0);
            let background_color =
                Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t;

            color *= background_color;
            break;
        }
    }

    color
}

fn main() -> Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let height = 720;
    let width = (height as f64 * aspect_ratio) as usize;
    let mut image = Image::new(width, height);

    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let ground_material = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let center_material = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let left_material = Dielectric::new(1.5);
    let right_material = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    let mut world = objects::ObjectList::new(vec![]);

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        ground_material,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        center_material,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        left_material.clone(),
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        left_material,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        right_material,
    ));

    let world = world;

    // Camera
    let cam_pos = Point3::new(-2.0, 2.0, 1.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let cam_dir = cam_pos - look_at;

    let vup = Point3::new(0.0, 1.0, 0.0);

    let focus_dist = cam_dir.length();
    let aperture = 0.0;

    let camera = Camera::new(
        Ray::new(cam_pos, cam_dir),
        vup,
        20.0,
        aspect_ratio,
        aperture,
        focus_dist,
    );

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

                pixel_color += ray_color(&ray, &world, max_depth);
            }

            *pixel = pixel_color / samples_per_pixel as f64;

            progress.inc(1);
            coz::progress!()
        });

    image.write_ppm("img.ppm")?;
    progress.finish();

    println!("Done! Took {}ms", now.elapsed().as_millis());

    Ok(())
}
