use crate::{
    color::Color,
    materials::{BoxedMaterial, Isotropic, Material},
    ray::Ray,
    textures::Texture,
    time::Time,
    vec3::Vec3,
};

use super::{BoxedObject, HitRecord, Object};

#[derive(Clone)]
pub struct ConstantMedium<'a> {
    boundary: BoxedObject<'a>,
    phase_function: BoxedMaterial<'a>,
    neg_inv_density: f32,
}

impl<'a> ConstantMedium<'a> {
    pub fn new(
        boundary: impl Object + 'a + Sync + Send,
        phase_function: impl Material + 'a + Sync + Send,
        density: f32,
    ) -> Self {
        Self::new_boxed(Box::new(boundary), Box::new(phase_function), density)
    }

    pub fn new_boxed(
        boundary: BoxedObject<'a>,
        phase_function: BoxedMaterial<'a>,
        density: f32,
    ) -> Self {
        Self {
            boundary,
            phase_function,
            neg_inv_density: -1.0 / density,
        }
    }

    #[allow(dead_code)]
    pub fn from_texture(
        boundary: impl Object + 'a + Sync + Send,
        texture: impl Texture + 'a + Sync + Send,
        density: f32,
    ) -> Self {
        Self::new(boundary, Isotropic::new(texture), density)
    }

    pub fn from_color(
        boundary: impl Object + 'a + Sync + Send,
        color: Color,
        density: f32,
    ) -> Self {
        Self::new(boundary, Isotropic::from_color(color), density)
    }
}

impl Object for ConstantMedium<'_> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<super::HitRecord> {
        let mut rec1 = self
            .boundary
            .hit(ray, -std::f32::INFINITY, std::f32::INFINITY)?;
        let mut rec2 = self
            .boundary
            .hit(ray, rec1.t + 0.0001, std::f32::INFINITY)?;

        if rec1.t < t_min {
            rec1.t = t_min;
        }
        if rec2.t > t_max {
            rec2.t = t_max;
        }
        if rec1.t >= rec2.t {
            return None;
        }
        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = ray.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand::random::<f32>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let point = ray.at(t);

        Some(HitRecord {
            point,
            normal: Vec3::new(1.0, 0.0, 0.0),
            material: &*self.phase_function,
            u: 0.0,
            v: 0.0,
            t,
            front_face: true,
        })
    }

    fn bounding_box(&self, timeframe: Time) -> Option<crate::aabb::AABB> {
        self.boundary.bounding_box(timeframe)
    }
}
