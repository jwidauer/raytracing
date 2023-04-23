mod dielectric;
mod diffusers;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
use dyn_clonable::clonable;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{color::Color, objects::HitRecord, ray::Ray};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

#[clonable]
pub trait Material: Clone {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}

pub type BoxedMaterial<'a> = Box<dyn Material + Send + Sync + 'a>;
