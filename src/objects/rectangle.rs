use thiserror::Error;

use crate::{
    aabb::AABB,
    materials::{BoxedMaterial, Material},
    ray::Ray,
    time::Time,
    vec3::Point3,
};

use super::{triangle::Triangle, HitRecord, Object};

#[derive(Error, Debug)]
pub enum RectangleError {
    #[error("The rectangle is not planar")]
    NotPlanar,
}

#[derive(Clone)]
pub struct Rectangle<'a> {
    triangle1: Triangle<'a>,
    triangle2: Triangle<'a>,
}

impl<'a> Rectangle<'a> {
    pub fn new(
        p0: Point3,
        p1: Point3,
        p2: Point3,
        p3: Point3,
        material: impl Material + 'a + Send + Sync,
    ) -> Result<Self, RectangleError> {
        if !((p1 - p0).cross(&(p2 - p0)).normalized() - (p2 - p0).cross(&(p3 - p0)).normalized())
            .near_zero()
        {
            return Err(RectangleError::NotPlanar);
        }

        let material = Box::new(material);
        let material = material as BoxedMaterial<'a>;
        Ok(Rectangle {
            triangle1: Triangle::new(p0, p1, p2, material.clone()),
            triangle2: Triangle::new(p0, p2, p3, material),
        })
    }

    pub fn from_points(
        points: [Point3; 4],
        material: impl Material + 'a + Send + Sync,
    ) -> Result<Self, RectangleError> {
        Self::new(points[0], points[1], points[2], points[3], material)
    }

    pub fn new_xy(
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
        z: f32,
        material: impl Material + Send + Sync + 'a,
    ) -> Self {
        let p0 = Point3::new(x0, y0, z);
        let p1 = Point3::new(x1, y0, z);
        let p2 = Point3::new(x1, y1, z);
        let p3 = Point3::new(x0, y1, z);

        Self::from_points([p0, p1, p2, p3], material).unwrap()
    }

    pub fn new_xz(
        x0: f32,
        x1: f32,
        z0: f32,
        z1: f32,
        y: f32,
        material: impl Material + Send + Sync + 'a,
    ) -> Self {
        let p0 = Point3::new(x0, y, z0);
        let p1 = Point3::new(x1, y, z0);
        let p2 = Point3::new(x1, y, z1);
        let p3 = Point3::new(x0, y, z1);

        Self::from_points([p0, p1, p2, p3], material).unwrap()
    }

    pub fn new_yz(
        y0: f32,
        y1: f32,
        z0: f32,
        z1: f32,
        x: f32,
        material: impl Material + Send + Sync + 'a,
    ) -> Self {
        let p0 = Point3::new(x, y0, z0);
        let p1 = Point3::new(x, y1, z0);
        let p2 = Point3::new(x, y1, z1);
        let p3 = Point3::new(x, y0, z1);

        Self::from_points([p0, p1, p2, p3], material).unwrap()
    }
}

impl Object for Rectangle<'_> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.triangle1
            .hit(ray, t_min, t_max)
            .or_else(|| self.triangle2.hit(ray, t_min, t_max))
    }

    fn bounding_box(&self, timeframe: Time) -> Option<AABB> {
        Some(AABB::surrounding_box(
            &self.triangle1.bounding_box(timeframe)?,
            &self.triangle2.bounding_box(timeframe)?,
        ))
    }
}

impl Default for Rectangle<'_> {
    fn default() -> Self {
        Self::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
            crate::materials::Lambertian::new(crate::color::Color::new(0.5, 0.5, 0.5)),
        )
        .unwrap()
    }
}
