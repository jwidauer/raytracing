use crate::{
    aabb::AABB,
    materials::{BoxedMaterial, Material},
    ray::Ray,
    time::Time,
    vec3::{Point3, Vec3},
};

use super::{HitRecord, Object, Transformable};

#[derive(Clone)]
pub struct Sphere<'a> {
    movement: Ray,
    rotation: Vec3,
    radius: f32,
    material: BoxedMaterial<'a>,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f32, material: impl Material + 'a + Send + Sync) -> Self {
        Self::new_boxed(center, radius, Box::new(material))
    }

    pub fn new_boxed(center: Point3, radius: f32, material: BoxedMaterial<'a>) -> Self {
        Self {
            movement: Ray::new(center, Vec3::zero()),
            rotation: Vec3::new(0., 0., 0.),
            radius,
            material,
        }
    }

    pub fn new_moving(
        center0: Point3,
        center1: Point3,
        timeframe: Time,
        radius: f32,
        material: impl Material + 'a + Send + Sync,
    ) -> Self {
        Self::new_moving_boxed(center0, center1, timeframe, radius, Box::new(material))
    }

    pub fn new_moving_boxed(
        center0: Point3,
        center1: Point3,
        timeframe: Time,
        radius: f32,
        material: BoxedMaterial<'a>,
    ) -> Self {
        let movement = Ray::new(
            center0,
            (center1 - center0) / (timeframe.end - timeframe.start),
        );
        Self {
            movement,
            rotation: Vec3::new(0., 0., 0.),
            radius,
            material,
        }
    }

    // Compute the UV coordinates of a point on the surface of a unit sphere.
    pub fn sphere_uv(&self, p: &Vec3) -> (f32, f32) {
        const PI: f32 = std::f32::consts::PI;

        // x = -cos(phi) * sin(theta)
        // y = -cos(theta)
        // z = sin(phi) * sin(theta)

        let p = p.rotate(self.rotation, self.rotation.length());

        let phi = (-p.z()).atan2(p.x()) + PI;
        let theta = (-p.y()).acos();

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Object for Sphere<'_> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.movement.at(ray.time());
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_disc = discriminant.sqrt();

        let calc_hit_record = |root: f32| {
            let p = ray.at(root);
            let normal = (p - self.movement.at(ray.time())) / self.radius;
            let (normal, front_face) = HitRecord::orient_towards_ray(ray, normal);
            let (u, v) = self.sphere_uv(&normal);
            Some(HitRecord {
                point: p,
                normal,
                t: root,
                front_face,
                material: self.material.as_ref(),
                u,
                v,
            })
        };

        let root = (-half_b - sqrt_disc) / a;
        if (t_min..t_max).contains(&root) {
            return calc_hit_record(root);
        }

        let root = (-half_b + sqrt_disc) / a;
        if (t_min..t_max).contains(&root) {
            return calc_hit_record(root);
        }

        None
    }

    fn bounding_box(&self, timeframe: Time) -> Option<crate::aabb::AABB> {
        Some(AABB::new(
            self.movement.at(timeframe.start) - Vec3::new(self.radius, self.radius, self.radius),
            self.movement.at(timeframe.end) + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}

impl Transformable for Sphere<'_> {
    fn translate(self, offset: Vec3) -> Self {
        let movement = self.movement.translate(offset);
        Self { movement, ..self }
    }

    fn rotate(self, axis: Vec3, angle: f32) -> Self {
        let movement = self.movement.rotate(axis, angle);

        // Rotate the sphere's UV coordinates to match the rotation of the sphere.
        let rotation = self.rotation + axis * angle;

        Self {
            movement,
            rotation,
            ..self
        }
    }

    fn scale(self, factor: f32) -> Self {
        let radius = self.radius * factor;
        Self { radius, ..self }
    }
}
