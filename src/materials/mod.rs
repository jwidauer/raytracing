mod dielectric;
mod diffuse_light;
mod diffusers;
mod lambertian;
mod metal;

use dyn_clonable::clonable;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{color::Color, objects::HitRecord, ray::Ray, vec3::Vec3};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

#[clonable]
pub trait Material: Clone {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
    fn emitted(&self, _u: f64, _v: f64, _point: &Vec3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub type BoxedMaterial<'a> = Box<dyn Material + Send + Sync + 'a>;
