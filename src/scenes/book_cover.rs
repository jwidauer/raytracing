use crate::objects::{BvhNode, Object};
use crate::textures::Checker;
use crate::time::Time;
use crate::{
    color::Color,
    materials::{Dielectric, Lambertian, Metal},
    objects::{ObjectList, Sphere},
    vec3::Point3,
};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub fn new(time: Time) -> impl Object {
    let mut world = ObjectList::new(vec![]);

    let checker = Checker::from_colors(Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
    let ground_material = Lambertian::from_texture(checker);
    let ground = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    world.add(ground);

    let mut random_spheres = ObjectList::new(vec![]);
    let mut rng = ChaCha8Rng::seed_from_u64(123);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center = Point3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere = if choose_mat < 0.8 {
                    // diffuse
                    let (r1, g1, b1, r2, g2, b2) = rng.gen();
                    let albedo = Color::new(r1, g1, b1) * Color::new(r2, g2, b2);
                    let center2 = center + Point3::new(0.0, rng.gen_range(0.0f32..0.2), 0.0);
                    Sphere::new_moving(center, center2, time, 0.2, Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::new(
                        rng.gen_range(0.5f32..1.0),
                        rng.gen_range(0.5f32..1.0),
                        rng.gen_range(0.5f32..1.0),
                    );
                    let fuzz = 0.5 * rng.gen::<f32>();
                    Sphere::new(center, 0.2, Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Sphere::new(center, 0.2, Dielectric::new(1.5))
                };

                random_spheres.add(sphere);
            }
        }
    }

    world.add(BvhNode::from_list(&random_spheres, time));

    let material1 = Dielectric::new(1.5);
    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1);
    world.add(sphere1);

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2);
    world.add(sphere2);

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3);
    world.add(sphere3);

    world
}
