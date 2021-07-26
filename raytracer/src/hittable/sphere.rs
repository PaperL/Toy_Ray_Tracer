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
pub struct Sphere {
    pub cen: Point3, // center
    pub r: f64,      // radius
    pub mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, mat_ptr: Rc<dyn Material>) -> Self {
        Self { cen, r, mat_ptr }
    }

    pub fn get_sphere_uv(p: Point3) -> (f64, f64) {
        let theta = f64::acos(-p.y);
        let phi = f64::atan2(-p.z, p.x) + PI;
        (phi / (2. * PI), theta / PI)
    }
}

impl Hittable for Sphere {
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
            mat_ptr: self.mat_ptr.clone(),
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

    fn bounding_box(&self, _time: f64, _dur: f64) -> Option<AABB> {
        Some(AABB::new(
            self.cen - Vec3::new(self.r, self.r, self.r),
            self.cen + Vec3::new(self.r, self.r, self.r),
        ))
    }
}
