use dyn_clonable::*;

mod bvh_node;
mod constant_medium;
mod cuboid;
mod object_list;
mod rectangle;
mod sphere;
mod triangle;

pub use bvh_node::BvhNode;
pub use constant_medium::ConstantMedium;
pub use cuboid::Cuboid;
pub use object_list::ObjectList;
pub use rectangle::Rectangle;
pub use sphere::Sphere;
pub use triangle::Triangle;

use crate::{aabb::AABB, materials::Material, ray::Ray, time::Time, vec3::Vec3};

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
    pub u: f32,
    pub v: f32,
}

impl HitRecord<'_> {
    /// Orients the normal vector towards the ray.
    /// This is necessary because we might be inside the object, in which case
    /// the normal vector would be pointing in the wrong direction.
    pub fn orient_towards_ray(ray: &Ray, outward_normal: Vec3) -> (Vec3, bool) {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        (normal, front_face)
    }
}

#[clonable]
pub trait Object: Clone {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, timeframe: Time) -> Option<AABB>;
}

pub type BoxedObject<'a> = Box<dyn Object + Send + Sync + 'a>;

pub trait Transformable {
    fn translate(self, offset: Vec3) -> Self;
    fn rotate(self, axis: Vec3, angle_rad: f32) -> Self;
    fn scale(self, factor: f32) -> Self;
}
