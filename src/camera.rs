use crate::rand_ext::rand;
use crate::settings::CameraSettings;
use crate::time::Time;
use crate::{ray::Ray, vec3::Vec3};

#[derive(Debug, Clone)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner_direction: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    time: Time,
}

impl Camera {
    pub fn new(
        view_ray: Ray,
        view_up: Vec3,
        vertical_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        time: Time,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = view_ray.direction().normalized();
        let u = view_up.cross(w).normalized();
        let v = w.cross(u);

        let origin = *view_ray.origin();
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
            time,
        }
    }

    pub fn from_settings(settings: CameraSettings) -> Camera {
        let view_ray = Ray::new(settings.cam_pos, settings.cam_pos - settings.look_at);
        Self::new(
            view_ray,
            settings.vup,
            settings.vfov,
            settings.aspect_ratio,
            settings.aperture,
            settings.focus_dist,
            settings.time,
        )
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new_time_based(
            self.origin + offset,
            self.lower_left_corner_direction + s * self.horizontal + t * self.vertical - offset,
            rand::random_range(self.time.start, self.time.end),
        )
    }
}
