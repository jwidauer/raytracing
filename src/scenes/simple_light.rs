use crate::{
    color::Color,
    materials::{Dielectric, DiffuseLight, Lambertian},
    objects::{Object, ObjectList, Rectangle, Sphere},
    textures::{ImageTexture, NoiseTexture},
    vec3::Point3,
};

pub fn new() -> impl Object {
    let mut objects = ObjectList::new(vec![]);

    let perlin_texture = NoiseTexture::new(4.);
    objects.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::from_texture(perlin_texture),
    ));

    objects.add(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Dielectric::from_color(Color::new(1.0, 0.7, 0.7), 1.5),
    ));
    objects.add(Sphere::new(
        Point3::new(-4.5, 2.0, 0.0),
        2.0,
        Lambertian::from_texture(ImageTexture::new("assets/earthmap.jpg")),
    ));
    objects.add(Rectangle::new_yz(
        3.0,
        5.0,
        1.0,
        3.0,
        5.0,
        DiffuseLight::from_color(Color::new(4.0, 0.5, 0.5)),
    ));

    let light_texture = DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0));
    objects.add(Sphere::new(Point3::new(-2.0, 8.0, 3.0), 2.0, light_texture));

    objects
}
