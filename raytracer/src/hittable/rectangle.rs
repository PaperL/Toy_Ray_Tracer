use std::rc::Rc;

use crate::{
    basic::{
        ray::Ray,
        vec3::{Point3, Vec3},
        INFINITESIMAL,
    },
    bvh::aabb::AABB,
    material::Material,
};

use super::{HitRecord, Hittable};

pub struct Rectangle {
    pub dir: u32,
    pub coo: [[f64; 2]; 3],
    pub mat_ptr: Rc<dyn Material>,
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
        mat_ptr: Rc<dyn Material>,
    ) -> Self {
        let mut tr = Rectangle {
            dir,
            coo: Default::default(),
            mat_ptr,
            di: Default::default(),
        };
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<super::HitRecord> {
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
            // xy
            0 => Vec3::new(0., 0., 1.),
            // yz
            1 => Vec3::new(1., 0., 0.),
            // xz
            2 => Vec3::new(0., 1., 0.),
            _ => panic!("Get unexpected dir in Rectangle::hit!"),
        };

        Some(HitRecord::new(
            (b1 - a1) / (a2 - a1),
            (b2 - a3) / (a4 - a3),
            t,
            ray,
            &outward_normal,
            self.mat_ptr.clone(),
        ))
    }

    fn bounding_box(&self, _time: f64, _dur: f64) -> Option<AABB> {
        Some(AABB {
            min: Point3::new(
                self.coo[0][0] + (if self.di[2] == 1 { -INFINITESIMAL } else { 0. }),
                self.coo[1][0] + (if self.di[2] == 2 { -INFINITESIMAL } else { 0. }),
                self.coo[2][0] + (if self.di[2] == 0 { -INFINITESIMAL } else { 0. }),
            ),
            max: Point3::new(
                self.coo[0][1] + (if self.di[2] == 1 { INFINITESIMAL } else { 0. }),
                self.coo[1][1] + (if self.di[2] == 2 { INFINITESIMAL } else { 0. }),
                self.coo[2][1] + (if self.di[2] == 0 { INFINITESIMAL } else { 0. }),
            ),
        })
    }
}
