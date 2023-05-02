use crate::vec3::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f32,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self::new_time_based(origin, direction, 0.)
    }

    pub fn new_time_based(origin: Point3, direction: Vec3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn translate(self, offset: Vec3) -> Self {
        Self::new(self.origin + offset, self.direction)
    }

    pub fn rotate(self, axis: Vec3, angle: f32) -> Self {
        let direction = self.direction.rotate(axis, angle);

        Self::new(self.origin, direction)
    }
}
