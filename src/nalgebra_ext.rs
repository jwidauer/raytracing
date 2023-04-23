use nalgebra::Vector3;

pub trait ReflectionExt {
    fn reflect(&self, normal: &Vector3<f64>) -> Vector3<f64>;
}

impl ReflectionExt for Vector3<f64> {
    fn reflect(&self, normal: &Vector3<f64>) -> Vector3<f64> {
        self - 2.0 * self.dot(normal) * normal
    }
}

pub trait RefractionExt {
    fn refract(&self, normal: &Vector3<f64>, ni_over_nt: f64) -> Vector3<f64>;
}

impl RefractionExt for Vector3<f64> {
    fn refract(&self, normal: &Vector3<f64>, ni_over_nt: f64) -> Vector3<f64> {
        let cos_theta = (-self).dot(normal).min(1.0);
        let r_out_perp = ni_over_nt * (self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.norm_squared()).abs().sqrt() * normal;

        r_out_perp + r_out_parallel
    }
}
