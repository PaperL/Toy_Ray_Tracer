use std::sync::Arc;

use crate::{
    bvh::aabb::AABB,
    hittable::{HitRecord, Hittable},
};

pub struct Flip {
    item: Arc<dyn Hittable>,
}

impl Flip {
    pub fn new(item: Arc<dyn Hittable>) -> Self {
        Self { item }
    }
}

impl Hittable for Flip {
    fn hit(&self, ray: &crate::basic::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec) = self.item.hit(&ray, t_min, t_max) {
            rec.front_face = !rec.front_face;

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, tm: f64, dur: f64) -> Option<AABB> {
        self.item.bounding_box(tm, dur)
    }
}
