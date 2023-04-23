use crate::{
    materials::Lambertian,
    objects::{Object, Sphere},
    textures,
    vec3::Point3,
};

pub fn new() -> impl Object {
    let earth_texture = textures::ImageTexture::new("assets/earthmap.jpg");
    let earth_surface = Lambertian::from_texture(earth_texture);
    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);

    globe
}
