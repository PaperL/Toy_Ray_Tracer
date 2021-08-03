use super::super::{HitRecord, Hittable};

use crate::{
    basic::{
        ray::Ray,
        vec3::{Point3, Vec3},
    },
    bvh::aabb::AABB,
};

#[derive(Clone)]
pub struct Translate<TH>
where
    TH: Hittable,
{
    pub obj: TH,
    pub offset: Vec3,
}

impl<TH: Hittable> Translate<TH> {
    pub fn new(obj: TH, offset: Vec3) -> Self {
        Self { obj, offset }
    }
}

impl<TH: Hittable> Hittable for Translate<TH> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.orig - self.offset, ray.dir, ray.tm);
        if let Some(mut rec) = self.obj.hit(&moved_ray, t_min, t_max) {
            rec.p += self.offset;
            rec.set_face_normal(&moved_ray, &rec.normal.clone());

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, tm: f64, dur: f64) -> Option<AABB> {
        self.obj
            .bounding_box(tm, dur)
            .map(|output_box| AABB::new(output_box.min + self.offset, output_box.max + self.offset))
    }

    fn pdf_value(&self, orig: &Point3, dir: &Vec3) -> f64 {
        self.obj.pdf_value(&(*orig - self.offset), dir)
    }

    fn rand_dir(&self, orig: &Vec3) -> Vec3 {
        self.obj.rand_dir(&(*orig - self.offset))
    }
}
