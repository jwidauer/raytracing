use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

use crate::{
    color::Color,
    materials::{Dielectric, DiffuseLight, Lambertian, Metal},
    objects::{BvhNode, ConstantMedium, Cuboid, Object, ObjectList, Rectangle, Sphere},
    textures::{ImageTexture, NoiseTexture},
    time::Time,
    vec3::{Point3, Vec3},
};

pub fn new(timeframe: Time) -> impl Object {
    let mut boxes = ObjectList::new(vec![]);

    let ground = Lambertian::new(Color::new(0.48, 0.83, 0.53));

    let mut rng = ChaCha8Rng::seed_from_u64(123);

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0f32..101.0);
            let z1 = z0 + w;

            boxes.add(Cuboid::bounded_by(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut objects = ObjectList::new(vec![]);

    objects.add(BvhNode::from_list(&boxes, timeframe));

    let light = DiffuseLight::from_color(Color::new(7., 7., 7.));
    objects.add(Rectangle::new_xz(123., 423., 147., 412., 554., light));

    let center1 = Point3::new(400., 400., 200.);
    let center2 = center1 + Vec3::new(30., 0., 0.);
    let material = Lambertian::new(Color::new(0.7, 0.3, 0.1));
    let moving_sphere = Sphere::new_moving(center1, center2, timeframe, 50., material);
    objects.add(moving_sphere);

    objects.add(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Dielectric::new(1.5),
    ));
    objects.add(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Metal::new(Color::new(0.8, 0.8, 0.9), 1.0),
    ));

    let boundary = Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
    objects.add(boundary.clone());
    objects.add(ConstantMedium::from_color(
        boundary,
        Color::new(0.2, 0.4, 0.9),
        0.2,
    ));
    let boundary = Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
    objects.add(ConstantMedium::from_color(
        boundary,
        Color::new(1.0, 1.0, 1.0),
        0.0001,
    ));

    let emat = Lambertian::from_texture(ImageTexture::new("assets/earthmap.jpg"));
    objects.add(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, emat));
    let pertext = NoiseTexture::new(0.1);
    objects.add(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::from_texture(pertext),
    ));

    let mut boxes = ObjectList::new(vec![]);
    let white = Lambertian::new(Color::new(0.73, 0.73, 0.73));
    let translation = Vec3::new(-100., 270., 395.);
    let ns = 1000;
    for _ in 0..ns {
        let (x, y, z) = rng.gen::<(f32, f32, f32)>();
        boxes.add(Sphere::new(
            Point3::new(x * 165.0, y * 165.0, z * 165.) + translation,
            10.0,
            white.clone(),
        ));
    }

    objects.add(BvhNode::from_list(&boxes, timeframe));

    objects
}
