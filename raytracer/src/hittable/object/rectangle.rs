use std::{f64::INFINITY, sync::Arc};

use rand::Rng;

use super::super::{HitRecord, Hittable};

use crate::{
    basic::{
        ray::Ray,
        vec3::{Point3, Vec3},
        INFINITESIMAL,
    },
    bvh::aabb::AABB,
    material::Material,
};

#[derive(Clone)]
pub struct Rectangle {
    pub dir: u32,
    pub coo: [[f64; 2]; 3],
    pub mat: Arc<dyn Material>,
    di: [usize; 3], // dimension index
}

impl Rectangle {
    pub fn new(
        dir: u32,
        a1: f64,
        a2: f64,
        a3: f64,
        a4: f64,
        k: f64,
        mat: Arc<dyn Material>,
    ) -> Self {
        let mut tr = Rectangle {
            dir,
            coo: Default::default(),
            mat,
            di: Default::default(),
        }; // temp rectangle

        match dir {
            // xy
            0 => tr.di = [0, 1, 2],
            // yz
            1 => tr.di = [1, 2, 0],
            // xz
            2 => tr.di = [0, 2, 1],
            _ => panic!("Get unexpected dir in Rectangle::new!"),
        }
        tr.coo[tr.di[0]][0] = a1;
        tr.coo[tr.di[0]][1] = a2;
        tr.coo[tr.di[1]][0] = a3;
        tr.coo[tr.di[1]][1] = a4;
        tr.coo[tr.di[2]][0] = k;
        tr.coo[tr.di[2]][1] = k;

        tr
    }
}

impl Hittable for Rectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let a1 = self.coo[self.di[0]][0];
        let a2 = self.coo[self.di[0]][1];
        let a3 = self.coo[self.di[1]][0];
        let a4 = self.coo[self.di[1]][1];
        let k = self.coo[self.di[2]][0];

        let t = (k - ray.orig[self.di[2]]) / ray.dir[self.di[2]];
        if t < t_min || t > t_max {
            return None;
        }

        let b1 = ray.orig[self.di[0]] + t * ray.dir[self.di[0]];
        let b2 = ray.orig[self.di[1]] + t * ray.dir[self.di[1]];
        if b1 < a1 || b1 > a2 || b2 < a3 || b2 > a4 {
            return None;
        }

        let outward_normal = match self.dir {
            0 => Vec3::new(0., 0., 1.), // xy
            1 => Vec3::new(1., 0., 0.), // yz
            2 => Vec3::new(0., 1., 0.), // xz
            _ => panic!("Get unexpected dir in Rectangle::hit!"),
        };

        Some(HitRecord::new(
            (b1 - a1) / (a2 - a1),
            (b2 - a3) / (a4 - a3),
            t,
            ray,
            &outward_normal,
            self.mat.clone(),
        ))
    }

    fn bounding_box(&self, _tm: f64, _dur: f64) -> Option<AABB> {
        let thickness = 10. * INFINITESIMAL;
        Some(AABB {
            min: Point3::new(
                self.coo[0][0] + (if self.di[2] == 0 { -thickness } else { 0. }),
                self.coo[1][0] + (if self.di[2] == 1 { -thickness } else { 0. }),
                self.coo[2][0] + (if self.di[2] == 2 { -thickness } else { 0. }),
            ),
            max: Point3::new(
                self.coo[0][1] + (if self.di[2] == 0 { thickness } else { 0. }),
                self.coo[1][1] + (if self.di[2] == 1 { thickness } else { 0. }),
                self.coo[2][1] + (if self.di[2] == 2 { thickness } else { 0. }),
            ),
        })
    }

    fn pdf_value(&self, orig: &Point3, dir: &Vec3) -> f64 {
        if let Some(rec) = self.hit(&Ray::new(*orig, *dir, 0.), INFINITESIMAL, INFINITY) {
            let a1 = self.coo[self.di[0]][0];
            let a2 = self.coo[self.di[0]][1];
            let a3 = self.coo[self.di[1]][0];
            let a4 = self.coo[self.di[1]][1];
            let area = (a2 - a1) * (a4 - a3);

            let dis_sqrd = rec.t.powi(2) * dir.length_squared();
            let cosine = (Vec3::dot(dir, &rec.normal) / dir.length()).abs();

            dis_sqrd / (cosine * area)
        } else {
            0.
        }
    }

    fn rand_dir(&self, orig: &Point3) -> Vec3 {
        let mut rnd = rand::thread_rng();
        let rand_point = Point3::new(
            if self.di[2] == 0 {
                self.coo[0][0]
            } else {
                rnd.gen_range(self.coo[0][0]..self.coo[0][1])
            },
            if self.di[2] == 1 {
                self.coo[1][0]
            } else {
                rnd.gen_range(self.coo[1][0]..self.coo[1][1])
            },
            if self.di[2] == 2 {
                self.coo[2][0]
            } else {
                rnd.gen_range(self.coo[2][0]..self.coo[2][1])
            },
        );

        rand_point - *orig
    }
}
