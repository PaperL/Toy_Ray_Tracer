use std::{
    f64::{INFINITY, NEG_INFINITY},
    rc::Rc,
};

use crate::{
    basic::{
        f64_equal, rand_1,
        vec3::{RGBColor, Vec3},
        INFINITESIMAL,
    },
    bvh::aabb::AABB,
    material::{isotropic::Isotropic, Material},
    texture::Texture,
};

use super::{HitRecord, Hittable};

pub struct ConstantMedium {
    pub boundary: Rc<dyn Hittable>,
    pub phase_function: Rc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Rc<dyn Hittable>, d: f64, a: Rc<dyn Texture>) -> Self {
        Self {
            boundary,
            neg_inv_density: -1. / d,
            phase_function: Rc::new(Isotropic::new(a)),
        }
    }

    pub fn new_from_color(boundary: Rc<dyn Hittable>, d: f64, c: RGBColor) -> Self {
        Self {
            boundary,
            neg_inv_density: -1. / d,
            phase_function: Rc::new(Isotropic::new_from_color(c)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &crate::basic::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec1) = self.boundary.hit(ray, NEG_INFINITY, INFINITY) {
            if let Some(mut rec2) = self.boundary.hit(ray, rec1.t + INFINITESIMAL, INFINITY) {
                if rec1.t < t_min {
                    rec1.t = t_min;
                }
                if rec2.t > t_max {
                    rec2.t = t_max;
                }
                if rec1.t > rec2.t || f64_equal(rec1.t, rec2.t) {
                    return None;
                }
                if rec1.t < 0. {
                    rec1.t = 0.;
                }

                let ray_len = ray.dir.length();
                let dis_inside_bound = (rec2.t - rec1.t) * ray_len;
                let hit_dis = self.neg_inv_density * f64::log2(rand_1());

                if hit_dis > dis_inside_bound {
                    return None;
                } else {
                    let t = rec1.t + hit_dis / ray_len;
                    return Some(HitRecord {
                        p: ray.at(t),
                        normal: Vec3::new(1., 0., 0.),
                        t,
                        front_face: true,
                        u: 0.,
                        v: 0.,
                        mat_ptr: self.phase_function.clone(),
                    });
                }
            }
        }
        None
    }

    fn bounding_box(&self, time: f64, dur: f64) -> Option<AABB> {
        self.boundary.bounding_box(time, dur)
    }
}
