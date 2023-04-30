use crate::vec3::Vec3;

#[allow(dead_code)]
pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let v = random_in_unit_sphere();
    if v.dot(normal) > 0.0 {
        v
    } else {
        -v
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    random_on_unit_sphere() * rand::random::<f32>().sqrt()
}

pub fn random_on_unit_sphere() -> Vec3 {
    let (theta, phi): (f32, f32) = rand::random();

    Vec3::new(
        theta.sin() * phi.cos(),
        theta.sin() * phi.sin(),
        theta.cos(),
    )
}

pub fn random_lambertian(normal: &Vec3) -> Vec3 {
    let (r1, r2): (f32, f32) = rand::random();
    let sin_theta = r1.sqrt();
    let cos_theta = (1.0 - sin_theta * sin_theta).sqrt();

    let phi = 2.0 * std::f32::consts::PI * r2;

    let x = phi.cos() * sin_theta;
    let y = phi.sin() * sin_theta;
    let z = cos_theta;

    let v = Vec3::new(x, y, z);

    // axis = normal x (0, 0, 1); angle = acos(normal dot (0, 0, 1))
    let axis = Vec3::new(normal.y(), -normal.x(), 0.0);
    let angle = normal.z().acos();

    v.rotate(&axis, angle)
}
