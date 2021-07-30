use std::sync::Arc;

use super::super::{HitRecord, Hittable};

use crate::{
    basic::{ray::Ray, vec3::Vec3},
    bvh::aabb::AABB,
};

pub struct Translate {
    pub item: Arc<dyn Hittable>,
    pub offset: Vec3,
}

impl Translate {
    pub fn new(hit_ptr: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self {
            item: hit_ptr,
            offset,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.orig - self.offset, ray.dir, ray.tm);
        if let Some(mut rec) = self.item.hit(&moved_ray, t_min, t_max) {
            rec.p += self.offset;
            rec.set_face_normal(&moved_ray, &rec.normal.clone());
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, tm: f64, dur: f64) -> Option<AABB> {
        // if let Some(output_box) = self.item.bounding_box(tm, dur) {
        //     Some(AABB::new(
        //         output_box.min + self.offset,
        //         output_box.max + self.offset,
        //     ))
        // } else {
        //     None
        // }
        self.item
            .bounding_box(tm, dur)
            .map(|output_box| AABB::new(output_box.min + self.offset, output_box.max + self.offset))
    }
}
