use std::array;

use rand::{thread_rng, Rng};

use crate::vec3::{Point3, Vec3};

const POINT_COUNT: usize = 256;

#[derive(Debug, Clone)]
pub struct Perlin {
    ranvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let ranvec: [Vec3; POINT_COUNT] =
            array::from_fn(|_| Vec3::random_range(-1., 1.).normalized());

        Self {
            ranvec,
            perm_x: Self::generate_permutation(),
            perm_y: Self::generate_permutation(),
            perm_z: Self::generate_permutation(),
        }
    }

    pub fn turb(&self, point: &Point3, depth: usize) -> f64 {
        let mut accum = 0.;
        let mut temp_p = *point;
        let mut weight = 1.;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }

        accum.abs()
    }

    pub fn noise(&self, point: &Point3) -> f64 {
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        let i = point.x().floor() as i64;
        let j = point.y().floor() as i64;
        let k = point.z().floor() as i64;
        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for (di, ci) in c.iter_mut().enumerate() {
            for (dj, cj) in ci.iter_mut().enumerate() {
                for (dk, c) in cj.iter_mut().enumerate() {
                    *c = self.ranvec[self.perm_x[((i + di as i64) & 255) as usize]
                        ^ self.perm_y[((j + dj as i64) & 255) as usize]
                        ^ self.perm_z[((k + dk as i64) & 255) as usize]]
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);

        let mut accum = 0.0;

        for (i, ci) in c.iter().enumerate() {
            for (j, cj) in ci.iter().enumerate() {
                for (k, c) in cj.iter().enumerate() {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1. - i as f64) * (1. - uu))
                        * (j as f64 * vv + (1. - j as f64) * (1. - vv))
                        * (k as f64 * ww + (1. - k as f64) * (1. - ww))
                        * c.dot(&weight_v);
                }
            }
        }

        accum
    }

    fn generate_permutation() -> [usize; POINT_COUNT] {
        let mut p: [usize; POINT_COUNT] = array::from_fn(|i| i);
        Self::permute(&mut p);
        p
    }

    fn permute(p: &mut [usize; POINT_COUNT]) {
        for i in (1..POINT_COUNT).rev() {
            let target = thread_rng().gen_range(0..i);
            p.swap(i, target);
        }
    }
}
