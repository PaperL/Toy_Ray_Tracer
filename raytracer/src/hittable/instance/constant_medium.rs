use std::f64::{INFINITY, NEG_INFINITY};

use super::super::{HitRecord, Hittable};

use crate::{
    basic::{
        f64_equal, rand_1,
        ray::Ray,
        vec3::{Point3, RGBColor, Vec3},
        INFINITESIMAL,
    },
    bvh::aabb::AABB,
    material::isotropic::Isotropic,
    texture::{solid_color::SolidColor, Texture},
};

#[derive(Clone)]
pub struct ConstantMedium<TH, TT>
where
    TH: Hittable,
    TT: Texture,
{
    pub boundary: TH,
    pub phase_function: Isotropic<TT>,
    neg_inv_density: f64,
}

impl<TH: Hittable, TT: Texture> ConstantMedium<TH, TT> {
    pub fn new(boundary: TH, d: f64, albedo: TT) -> Self {
        Self {
            boundary,
            neg_inv_density: -1. / d,
            phase_function: Isotropic::new(albedo),
        }
    }
}

impl<TH: Hittable> ConstantMedium<TH, SolidColor> {
    pub fn new_from_color(boundary: TH, d: f64, color_value: RGBColor) -> Self {
        Self {
            boundary,
            neg_inv_density: -1. / d,
            phase_function: Isotropic::new_from_color(color_value),
        }
    }
}

impl<TH: Hittable, TT: Texture> Hittable for ConstantMedium<TH, TT> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut hit_rec_1) = self.boundary.hit(ray, NEG_INFINITY, INFINITY) {
            if let Some(mut hit_rec_2) =
                self.boundary
                    .hit(ray, hit_rec_1.t + INFINITESIMAL, INFINITY)
            {
                if hit_rec_1.t < t_min {
                    hit_rec_1.t = t_min;
                }
                if hit_rec_2.t > t_max {
                    hit_rec_2.t = t_max;
                }
                if hit_rec_1.t < 0. {
                    hit_rec_1.t = 0.;
                }
                if hit_rec_1.t > hit_rec_2.t || f64_equal(hit_rec_1.t, hit_rec_2.t) {
                    return None;
                }

                let ray_len = ray.dir.length();
                let dis_inside_bound = (hit_rec_2.t - hit_rec_1.t) * ray_len;
                let hit_dis = self.neg_inv_density * f64::log2(rand_1());

                if hit_dis > dis_inside_bound {
                    return None;
                } else {
                    let t = hit_rec_1.t + hit_dis / ray_len;
                    return Some(HitRecord::new(
                        0.,
                        0.,
                        t,
                        ray,
                        &Vec3::new(1., 0., 0.),
                        &self.phase_function,
                    ));
                }
            }
        }
        None
    }

    fn bounding_box(&self, tm: f64, dur: f64) -> Option<AABB> {
        self.boundary.bounding_box(tm, dur)
    }

    fn pdf_value(&self, orig: &Point3, dir: &Vec3) -> f64 {
        self.boundary.pdf_value(orig, dir)
    }

    fn rand_dir(&self, orig: &Vec3) -> Vec3 {
        self.boundary.rand_dir(orig)
    }
}
