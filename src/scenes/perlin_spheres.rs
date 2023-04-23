use crate::{
    materials::Lambertian,
    objects::{Object, ObjectList, Sphere},
    textures::Noise,
    vec3::Point3,
};

pub fn new() -> impl Object {
    let mut objects = ObjectList::new(vec![]);

    let perlin_texture = Noise::new(4.);
    objects.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::from_texture(perlin_texture.clone()),
    ));

    objects.add(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::from_texture(perlin_texture),
    ));

    objects
}
