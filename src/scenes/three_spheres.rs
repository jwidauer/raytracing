use crate::{
    color::Color,
    materials::{Dielectric, Lambertian, Metal},
    objects::{ObjectList, Sphere},
    vec3::Point3,
};

pub fn new() -> ObjectList<'static> {
    let ground_material = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let center_material = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let left_material = Dielectric::new(1.5);
    let right_material = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    let mut world = ObjectList::new(vec![]);

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        ground_material,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        center_material,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        left_material.clone(),
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        left_material,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        right_material,
    ));

    world
}
