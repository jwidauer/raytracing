use crate::{
    color::Color,
    materials::Lambertian,
    objects::{Object, ObjectList, Sphere},
    textures::Checker,
    vec3::Point3,
};

pub fn new() -> impl Object {
    let mut world = ObjectList::new(vec![]);

    let checker = Checker::from_colors(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));

    world.add(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::from_texture(checker.clone()),
    ));

    world.add(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::from_texture(checker),
    ));

    world
}
