use crate::{
    color::Color,
    materials::{DiffuseLight, Lambertian},
    objects::{Cuboid, Object, ObjectList, Rectangle, Transformable},
    vec3::{Point3, Vec3},
};

pub fn new() -> impl Object {
    let red = Lambertian::new(Color::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(Color::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0));

    let mut objects = ObjectList::new(vec![]);
    objects.add(Rectangle::new_yz(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
    ));
    objects.add(Rectangle::new_yz(0.0, 555.0, 0.0, 555.0, 0.0, red.clone()));
    objects.add(Rectangle::new_xz(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light.clone(),
    ));
    objects.add(Rectangle::new_xz(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    ));
    objects.add(Rectangle::new_xz(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ));
    objects.add(Rectangle::new_xy(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ));

    let box1 = Cuboid::bounded_by(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    )
    .rotate(Vec3::new(0., 1., 0.), -15f32.to_radians())
    .translate(Vec3::new(265.0, 0.0, 295.0));

    objects.add(box1);

    let box2 = Cuboid::bounded_by(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    )
    .rotate(Vec3::new(0., 1., 0.), 18f32.to_radians())
    .translate(Vec3::new(130.0, 0.0, 65.0));

    objects.add(box2);

    objects
}
