use std::f64::{consts::PI, INFINITY};

use super::super::{HitRecord, Hittable};

use crate::{
    basic::{
        clamp_hoi,
        onb::ONB,
        ray::Ray,
        vec3::{Point3, Vec3},
        INFINITESIMAL,
    },
    bvh::aabb::AABB,
    material::Material,
};

#[derive(Clone)]
pub struct Sphere<TM>
where
    TM: Material,
{
    pub cen: Point3, // center
    pub r: f64,      // radius
    pub mat: TM,     // material
}

impl<TM: Material> Sphere<TM> {
    pub fn new(cen: Point3, r: f64, mat: TM) -> Self {
        Self { cen, r, mat }
    }

    pub fn get_sphere_uv(p: Point3) -> (f64, f64) {
        let theta = f64::acos(-p.y);
        let mut phi = f64::atan2(-p.z, p.x) + PI;
        if phi.is_sign_negative() {
            phi += PI;
        }

        (
            clamp_hoi(phi / (2. * PI), 0., 1.),
            clamp_hoi(theta / PI, 0., 1.),
        )
    }
}

impl<TM: Material> Hittable for Sphere<TM> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig - self.cen;
        let a = ray.dir.length_squared();
        let half_b = Vec3::dot(&oc, &ray.dir);
        let c = oc.length_squared() - self.r.powi(2);

        let discriminant = half_b.powi(2) - (a * c);
        if discriminant < 0. {
            return None;
        }
        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let mut rec = HitRecord {
            p: ray.at(root),
            normal: Vec3::default(),
            mat: &self.mat,
            t: root,
            front_face: bool::default(),
            u: 0.,
            v: 0.,
        };
        let outward_normal = (rec.p - self.cen) / self.r;
        rec.set_face_normal(ray, &outward_normal);
        let uv = Self::get_sphere_uv(outward_normal);
        rec.u = uv.0;
        rec.v = uv.1;

        Some(rec)
    }

    fn bounding_box(&self, _tm: f64, _dur: f64) -> Option<AABB> {
        Some(AABB::new(
            self.cen - Vec3::new(self.r, self.r, self.r),
            self.cen + Vec3::new(self.r, self.r, self.r),
        ))
    }

    fn pdf_value(&self, orig: &Point3, dir: &Vec3) -> f64 {
        if let Some(_hit_rec) = self.hit(&Ray::new(*orig, *dir, 0.), INFINITESIMAL, INFINITY) {
            let cos_theta_max = (1. - self.r.powi(2) / (self.cen - *orig).length_squared()).sqrt();
            let solid_angle = 2. * PI * (1. - cos_theta_max);

            Self::map_to(1. / solid_angle, 20., 5.)
        } else {
            Self::map_to(1000., 20., 5.)
        }
    }

    fn rand_dir(&self, orig: &Vec3) -> Vec3 {
        let dir = self.cen - *orig;
        let dis_sqrd = dir.length_squared();
        let uvw = ONB::build_from_w(&dir);

        uvw.local(&Vec3::rand_to_sphere(self.r, dis_sqrd))
    }
}
