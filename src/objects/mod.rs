use dyn_clonable::*;

mod bvh_node;
mod object_list;
mod sphere;

pub use object_list::ObjectList;
pub use sphere::Sphere;

use crate::{aabb::AABB, materials::Material, ray::Ray, vec3::Vec3};

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl HitRecord<'_> {
    /// Orients the normal vector towards the ray.
    /// This is necessary because we might be inside the object, in which case
    /// the normal vector would be pointing in the wrong direction.
    pub fn orient_towards_ray(ray: &Ray, outward_normal: Vec3) -> (Vec3, bool) {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}

type BoxedObject<'a> = Box<dyn Object + Send + Sync + 'a>;
