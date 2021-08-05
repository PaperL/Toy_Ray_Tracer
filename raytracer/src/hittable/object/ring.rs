use std::f64::{consts::PI, INFINITY};

use rand::{prelude::ThreadRng, Rng};

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
pub struct Ring<TM>
where
    TM: Material,
{
    pub r: f64,  // radius
    pub t: f64,  // thickness
    pub mat: TM, // material

    ds_min: f64, // distance squared min
    ds_max: f64, // distance squared max
}

impl<TM: Material> Ring<TM> {
    pub fn new(r: f64, t: f64, mat: TM) -> Self {
        Self {
            r,
            t,
            mat,
            ds_min: (r - t).powi(2),
            ds_max: (r + t).powi(2),
        }
    }
}

impl<TM: Material> Hittable for Ring<TM> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = -ray.orig.y / ray.dir.y;
        if t.is_nan() || t < t_min || t > t_max {
            None
        } else {
            let p = ray.orig + ray.dir * t;
            let d = p.x.powi(2) + p.z.powi(2);
            if d < self.ds_min || d > self.ds_max {
                None
            } else {
                Some(HitRecord::new(
                    0.,
                    0.,
                    t,
                    ray,
                    &Vec3::new(0., 1., 0.),
                    &self.mat,
                ))
            }
        }
    }

    fn bounding_box(&self, _tm: f64, _dur: f64) -> Option<AABB> {
        let l = self.r + self.t;
        let thickness = 0.001;

        Some(AABB::new(
            Point3::new(-l, -thickness, -l),
            Point3::new(l, thickness, l),
        ))
    }

    fn pdf_value(&self, orig: &Point3, dir: &Vec3) -> f64 {
        if let Some(_hit_rec) = self.hit(&Ray::new(*orig, *dir, 0.), INFINITESIMAL, INFINITY) {
            let cos_theta_max = (1. - self.r.powi(2) / (*orig).length_squared()).sqrt();
            let solid_angle = 2. * PI * (1. - cos_theta_max);

            Self::map_to(1. / solid_angle, 20., 5.)
        } else {
            Self::map_to(1000., 20., 5.)
        }
    }

    fn rand_dir(&self, orig: &Vec3) -> Vec3 {
        let mut rnd: ThreadRng = rand::thread_rng();
        let radian = rnd.gen::<f64>() * 2. * PI;
        let p = Vec3::new(f64::cos(radian), 0., f64::sin(radian))
            * (self.r - self.t + 2. * self.t * rnd.gen::<f64>());

        p - *orig
    }
}

#[derive(Clone)]
pub struct BrokenRing<TM>
where
    TM: Material,
{
    pub r: f64,  // radius
    pub t: f64,  // thickness
    pub mat: TM, // material

    ds_min: f64, // distance squared min
    ds_max: f64, // distance squared max

    pub point_list: Vec<f64>,
}

impl<TM: Material> BrokenRing<TM> {
    pub fn new(r: f64, t: f64, point_list: Vec<f64>, mat: TM) -> Self {
        Self {
            r,
            t,
            mat,
            ds_min: (r - t).powi(2),
            ds_max: (r + t).powi(2),
            point_list,
        }
    }
}

impl<TM: Material> Hittable for BrokenRing<TM> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = -ray.orig.y / ray.dir.y;
        if t.is_nan() || t < t_min || t > t_max {
            None
        } else {
            let p = ray.orig + ray.dir * t;
            let d = p.x.powi(2) + p.z.powi(2);
            if d < self.ds_min || d > self.ds_max {
                None
            } else {
                let mut radian = if p.x.abs() < INFINITESIMAL {
                    if p.z.is_sign_positive() {
                        PI / 2.
                    } else {
                        -PI / 2.
                    }
                } else if p.x.is_sign_positive() {
                    f64::atan(p.z / p.x)
                } else {
                    f64::atan(p.z / p.x) + PI
                };

                if radian.is_sign_negative() {
                    radian += 2. * PI;
                }

                let mut cnt = 0;
                for k in &self.point_list {
                    if radian < *k {
                        break;
                    } else {
                        cnt += 1;
                    }
                }

                if cnt % 2 != 0
                    || (self.point_list.len() % 2 == 0
                        && (radian + 2. * PI < *self.point_list.last().unwrap()))
                {
                    Some(HitRecord::new(
                        radian / 2. / PI,
                        0.,
                        t,
                        ray,
                        &Vec3::new(0., 1., 0.),
                        &self.mat,
                    ))
                } else {
                    None
                }
            }
        }
    }

    fn bounding_box(&self, _tm: f64, _dur: f64) -> Option<AABB> {
        let l = self.r + self.t;
        let thickness = 0.001;

        Some(AABB::new(
            Point3::new(-l, -thickness, -l),
            Point3::new(l, thickness, l),
        ))
    }

    fn pdf_value(&self, orig: &Point3, dir: &Vec3) -> f64 {
        if let Some(_hit_rec) = self.hit(&Ray::new(*orig, *dir, 0.), INFINITESIMAL, INFINITY) {
            let cos_theta_max = (1. - self.r.powi(2) / (*orig).length_squared()).sqrt();
            let solid_angle = 2. * PI * (1. - cos_theta_max);

            Self::map_to(1. / solid_angle, 20., 5.)
        } else {
            Self::map_to(1000., 20., 5.)
        }
    }

    fn rand_dir(&self, orig: &Vec3) -> Vec3 {
        let mut rnd: ThreadRng = rand::thread_rng();
        let radian = rnd.gen::<f64>() * 2. * PI;
        let p = Vec3::new(f64::cos(radian), 0., f64::sin(radian))
            * (self.r - self.t + 2. * self.t * rnd.gen::<f64>());

        p - *orig
    }
}
