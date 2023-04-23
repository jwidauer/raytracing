use crate::{
    color::Color,
    materials::{Dielectric, Lambertian, Material, Metal},
    objects::{ObjectList, Sphere},
    vec3::Point3,
};

pub fn new() -> ObjectList<'static> {
    let mut world = ObjectList::new(vec![]);

    let ground_material = Lambertian::new(Color::new(0.48, 0.83, 0.53));
    let ground = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    world.add(ground);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rand::random();
            let center = Point3::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Box<dyn Material + Send + Sync> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    Box::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = 0.5 * rand::random::<f64>();
                    Box::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Box::new(Dielectric::new(1.5))
                };

                let sphere = Sphere::new_boxed(center, 0.2, sphere_material);
                world.add(sphere);
            }
        }
    }

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
