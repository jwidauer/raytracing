use crate::rand_ext::rand;
use crate::{ray::Ray, vec3::Vec3};

#[derive(Debug, Clone)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner_direction: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    time0: f64,
    time1: f64,
}

impl Camera {
    pub fn new(
        view_ray: Ray,
        view_up: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = view_ray.direction().normalized();
        let u = view_up.cross(&w).normalized();
        let v = w.cross(&u);

        let origin = view_ray.origin().clone();
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner_direction = -horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner_direction,
            lens_radius,
            u,
            v,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new_time_based(
            self.origin + offset,
            self.lower_left_corner_direction + s * self.horizontal + t * self.vertical - offset,
            rand::random_range(self.time0, self.time1),
        )
    }
}