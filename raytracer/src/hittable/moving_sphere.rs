use super::{HitRecord, Hittable};

use crate::{
    basic::{
        ray::Ray,
        vec3::{Point3, Vec3},
    },
    bvh::aabb::AABB,
    material::Material,
};
use std::f64::consts::PI;

use std::rc::Rc;

#[derive(Clone)]
pub struct MovingSphere {
    pub cen0: Point3,
    pub mov: Vec3, // displacement vector
    // pub cen1: Point3,
    pub tm0: f64,
    pub dur: f64, // movement duration
    // pub tm1: f64,
    pub r: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        cen0: Point3,
        cen1: Point3,
        tm0: f64,
        tm1: f64,
        r: f64,
        mat_ptr: Rc<dyn Material>,
    ) -> Self {
        Self {
            cen0,
            mov: cen1 - cen0,
            tm0,
            dur: tm1 - tm0,
            r,
            mat_ptr,
        }
    }

    fn center(&self, tm: f64) -> Point3 {
        self.cen0 + (self.mov) * ((tm - self.tm0) / (self.dur))
    }

    pub fn get_sphere_uv(p: Point3) -> (f64, f64) {
        let theta = f64::acos(-p.y);
        let phi = f64::atan2(-p.z, p.x) + PI;
        (phi / (2. * PI), theta / PI)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let cen = self.center(ray.tm);
        let oc = ray.orig - cen;
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
            mat_ptr: self.mat_ptr.clone(),
            t: root,
            front_face: bool::default(),
            u: 0.,
            v: 0.,
        };
        // rec.t = root;
        // rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - cen) / self.r;
        rec.set_face_normal(ray, &outward_normal);
        let uv = Self::get_sphere_uv(outward_normal);
        rec.u = uv.0;
        rec.v = uv.1;

        Some(rec)
    }

    fn bounding_box(&self, time: f64, dur: f64) -> Option<AABB> {
        Some(AABB::surrounding_box(
            &AABB::new(
                self.center(time) - Vec3::new(self.r, self.r, self.r),
                self.center(time) + Vec3::new(self.r, self.r, self.r),
            ),
            &AABB::new(
                self.center(time + dur) - Vec3::new(self.r, self.r, self.r),
                self.center(time + dur) + Vec3::new(self.r, self.r, self.r),
            ),
        ))
    }
}
