use std::mem::swap;

use crate::basic::{min_f64, ray::Ray, vec3::Point3};

#[derive(Default, Clone, Copy)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..3 {
            let k = 1. / ray.dir[i];
            let mut t0 = (self.min[i] - ray.orig[i]) * k;
            let mut t1 = (self.max[i] - ray.orig[i]) * k;
            if k < 0. {
                swap(&mut t0, &mut t1);
            }
            t_min = min_f64(t_min, t0);
            t_max = min_f64(t_max, t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(box0: &Self, box1: &Self) -> Self {
        AABB::new(
            Point3::new(
                min_f64(box0.min.x, box1.min.x), // implicit deref, equal to "(*box0).min.x"
                min_f64(box0.min.y, box1.min.y),
                min_f64(box0.min.z, box1.min.z),
            ),
            Point3::new(
                min_f64(box0.max.x, box1.max.x),
                min_f64(box0.max.y, box1.max.y),
                min_f64(box0.max.z, box1.max.z),
            ),
        )
    }
}
