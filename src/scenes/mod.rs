mod book_cover;
mod globe;
mod perlin_spheres;
mod three_spheres;
mod two_spheres;

use crate::{color::Color, objects::BoxedObject, ray::Ray, time::Time};

pub struct Scene<'a> {
    objects: BoxedObject<'a>,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum SceneType {
    TwoSpheres,
    ThreeSpheres,
    BookCover,
    PerlinSpheres,
    Globe,
}

impl Scene<'_> {
    pub fn new(scene_type: SceneType, time: Time) -> Self {
        match scene_type {
            SceneType::TwoSpheres => Self {
                objects: Box::new(two_spheres::new(time)),
            },
            SceneType::ThreeSpheres => Self {
                objects: Box::new(three_spheres::new(time)),
            },
            SceneType::BookCover => Self {
                objects: Box::new(book_cover::new(time)),
            },
            SceneType::PerlinSpheres => Self {
                objects: Box::new(perlin_spheres::new()),
            },
            SceneType::Globe => Self {
                objects: Box::new(globe::new()),
            },
        }
    }

    pub fn ray_color(&self, r: &Ray, max_depth: usize) -> Color {
        let mut color = Color::new(1.0, 1.0, 1.0);
        let mut r = r.clone();
        for _ in 0..max_depth {
            if let Some(rec) = self.objects.hit(&r, 0.001, f64::INFINITY) {
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
}
