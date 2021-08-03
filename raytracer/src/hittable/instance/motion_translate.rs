use super::super::{HitRecord, Hittable};

use crate::{
    basic::{ray::Ray, vec3::Vec3},
    bvh::aabb::AABB,
};

#[derive(Clone)]
pub struct MotionTranslate<TH>
where
    TH: Hittable,
{
    pub obj: TH,
    pub mov: Vec3, // 位移
    pub tm: f64,   // 出现 & 运动开始时间
    pub dur: f64,  // 消失 & 运动停止时间
}

impl<TH: Hittable> MotionTranslate<TH> {
    pub fn new(obj: TH, mov: Vec3, tm: f64, dur: f64) -> Self {
        Self { obj, mov, tm, dur }
    }
}

impl<TH: Hittable> Hittable for MotionTranslate<TH> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let k = (ray.tm - self.tm) / self.dur;
        if k.is_sign_negative() || k > 1. {
            return None;
        }
        let offset = self.mov * k;

        let moved_ray = Ray::new(ray.orig - offset, ray.dir, ray.tm);
        if let Some(mut rec) = self.obj.hit(&moved_ray, t_min, t_max) {
            rec.p += offset;
            rec.set_face_normal(&moved_ray, &rec.normal.clone());

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, tm: f64, dur: f64) -> Option<AABB> {
        let obj_box = self.obj.bounding_box(tm, dur).unwrap();
        Some(AABB::surrounding_box(
            &obj_box,
            &AABB::new(obj_box.min + self.mov, obj_box.max + self.mov),
        ))
    }
}
