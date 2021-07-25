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

pub struct XYRectangle {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl Hittable for XYRectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<super::HitRecord> {
        let t = (self.k - ray.orig.z) / ray.dir.z;
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.orig.x + t * ray.dir.x;
        let y = ray.orig.y + t * ray.dir.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some(HitRecord::new(
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
            t,
            ray,
            &Vec3::new(0., 0., 1.),
            self.mat_ptr.clone(),
        ))
    }

    fn bounding_box(&self, _time: f64, _dur: f64) -> Option<AABB> {
        Some(AABB {
            min: Point3::new(self.x0, self.y0, self.k - INFINITESIMAL),
            max: Point3::new(self.x1, self.y1, self.k + INFINITESIMAL),
        })
    }
}
