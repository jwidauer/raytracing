mod book_cover;
mod three_spheres;

use crate::{
    color::Color,
    objects::{Object, ObjectList},
    ray::Ray,
};

pub struct Scene<'a> {
    objects: ObjectList<'a>,
}

#[allow(dead_code)]
pub enum SceneType {
    ThreeSpheres,
    BookCover,
}

impl Scene<'_> {
    pub fn new(scene_type: SceneType) -> Self {
        match scene_type {
            SceneType::ThreeSpheres => Self {
                objects: three_spheres::new(),
            },
            SceneType::BookCover => Self {
                objects: book_cover::new(),
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
