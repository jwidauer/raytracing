use crate::{
    materials::Material,
    ray::Ray,
    time::Time,
    vec3::{Point3, Vec3},
};

use super::{HitRecord, Object, Rectangle, Transformable};

#[derive(Clone)]
pub struct Cuboid<'a> {
    faces: [Rectangle<'a>; 6],
}

impl<'a> Cuboid<'a> {
    pub fn new(corners: &[Point3; 8], material: impl Material + 'a + Send + Sync + Clone) -> Self {
        let faces: [Rectangle; 6] = [
            Rectangle::new(
                corners[0],
                corners[1],
                corners[2],
                corners[3],
                material.clone(),
            )
            .unwrap(),
            Rectangle::new(
                corners[4],
                corners[5],
                corners[6],
                corners[7],
                material.clone(),
            )
            .unwrap(),
            Rectangle::new(
                corners[0],
                corners[1],
                corners[5],
                corners[4],
                material.clone(),
            )
            .unwrap(),
            Rectangle::new(
                corners[2],
                corners[3],
                corners[7],
                corners[6],
                material.clone(),
            )
            .unwrap(),
            Rectangle::new(
                corners[0],
                corners[3],
                corners[7],
                corners[4],
                material.clone(),
            )
            .unwrap(),
            Rectangle::new(corners[1], corners[2], corners[6], corners[5], material).unwrap(),
        ];

        Self { faces }
    }

    pub fn bounded_by(
        min: Point3,
        max: Point3,
        material: impl Material + 'a + Send + Sync + Clone,
    ) -> Self {
        let corners = [
            min,
            Point3::new(max.x(), min.y(), min.z()),
            Point3::new(max.x(), max.y(), min.z()),
            Point3::new(min.x(), max.y(), min.z()),
            Point3::new(min.x(), min.y(), max.z()),
            Point3::new(max.x(), min.y(), max.z()),
            max,
            Point3::new(min.x(), max.y(), max.z()),
        ];

        Self::new(&corners, material)
    }
}

impl Object for Cuboid<'_> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;

        for face in self.faces.iter() {
            if let Some(record) = face.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }

        hit_record
    }

    fn bounding_box(&self, timeframe: Time) -> Option<crate::aabb::AABB> {
        let mut output_box = self.faces[0].bounding_box(timeframe).unwrap();

        for face in self.faces.iter() {
            output_box = crate::aabb::AABB::surrounding_box(
                &output_box,
                &face.bounding_box(timeframe).unwrap(),
            );
        }

        Some(output_box)
    }
}

impl Transformable for Cuboid<'_> {
    fn translate(self, offset: Vec3) -> Self {
        let faces = self
            .faces
            .into_iter()
            .map(|face| face.translate(offset))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|v: Vec<_>| panic!("Expected 6 faces, got {}", v.len()));

        Self { faces }
    }

    fn rotate(self, axis: Vec3, angle_rad: f32) -> Self {
        let faces = self
            .faces
            .into_iter()
            .map(|face| face.rotate(axis, angle_rad))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|v: Vec<_>| panic!("Expected 6 faces, got {}", v.len()));

        Self { faces }
    }

    fn scale(self, factor: f32) -> Self {
        let faces = self
            .faces
            .into_iter()
            .map(|face| face.scale(factor))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap_or_else(|v: Vec<_>| panic!("Expected 6 faces, got {}", v.len()));

        Self { faces }
    }
}
