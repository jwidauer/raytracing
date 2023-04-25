mod dielectric;
mod diffuse_light;
mod diffusers;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{color::Color, objects::HitRecord, ray::Ray, vec3::Vec3};

#[derive(Clone)]
pub enum MaterialEnum {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

impl MaterialEnum {
    pub fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        match self {
            MaterialEnum::Lambertian(material) => material.scatter(ray, hit_record),
            MaterialEnum::Metal(material) => material.scatter(ray, hit_record),
            MaterialEnum::Dielectric(material) => material.scatter(ray, hit_record),
            MaterialEnum::DiffuseLight(material) => material.scatter(ray, hit_record),
        }
    }

    pub fn emitted(&self, u: f64, v: f64, point: &Vec3) -> Color {
        match self {
            MaterialEnum::DiffuseLight(material) => material.emitted(u, v, point),
            _ => Color::new(0.0, 0.0, 0.0),
        }
    }
}

impl From<Lambertian> for MaterialEnum {
    fn from(material: Lambertian) -> Self {
        Self::Lambertian(material)
    }
}

impl From<Metal> for MaterialEnum {
    fn from(material: Metal) -> Self {
        Self::Metal(material)
    }
}

impl From<Dielectric> for MaterialEnum {
    fn from(material: Dielectric) -> Self {
        Self::Dielectric(material)
    }
}

impl From<DiffuseLight> for MaterialEnum {
    fn from(material: DiffuseLight) -> Self {
        Self::DiffuseLight(material)
    }
}

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
    fn emitted(&self, _u: f64, _v: f64, _point: &Vec3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}
