mod book_cover;
mod globe;
mod perlin_spheres;
mod simple_light;
mod three_spheres;
mod two_spheres;

use crate::{color::Color, objects::BoxedObject, ray::Ray, time::Time};

pub struct Scene<'a> {
    objects: BoxedObject<'a>,
    background: Color,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum SceneType {
    TwoSpheres,
    ThreeSpheres,
    BookCover,
    PerlinSpheres,
    Globe,
    SimpleLight,
}

impl Scene<'_> {
    pub fn new(scene_type: SceneType, time: Time, background: Color) -> Self {
        let objects: BoxedObject = match scene_type {
            SceneType::TwoSpheres => Box::new(two_spheres::new(time)),
            SceneType::ThreeSpheres => Box::new(three_spheres::new(time)),
            SceneType::BookCover => Box::new(book_cover::new(time)),
            SceneType::PerlinSpheres => Box::new(perlin_spheres::new()),
            SceneType::Globe => Box::new(globe::new()),
            SceneType::SimpleLight => Box::new(simple_light::new()),
        };

        Self {
            objects,
            background,
        }
    }

    pub fn ray_color(&self, ray: &Ray, max_depth: usize) -> Color {
        let mut color = Color::new(1.0, 1.0, 1.0);
        let mut ray = ray.clone();
        for _ in 0..max_depth {
            let hr = match self.objects.hit(&ray, 0.001, f64::INFINITY) {
                Some(hr) => hr,
                None => {
                    // Background
                    color *= self.background;
                    break;
                }
            };

            let emitted = hr.material.emitted(hr.u, hr.v, &hr.point);

            match hr.material.scatter(&ray, &hr) {
                Some(scatter) => {
                    color = emitted + color * scatter.attenuation;
                    ray = scatter.scattered;
                }
                None => {
                    // Hit a light source or got absorbed
                    color *= emitted;
                    break;
                }
            }
        }

        color
    }
}
