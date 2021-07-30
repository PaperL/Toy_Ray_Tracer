use std::{
    f64::{INFINITY, NEG_INFINITY},
    sync::Arc,
};

use super::super::{HitRecord, Hittable};

use crate::{
    basic::{
        degree_to_radian,
        ray::Ray,
        vec3::{Point3, Vec3},
    },
    bvh::aabb::AABB,
};

pub struct RotateY {
    item: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    aabb_box: AABB,
}

impl RotateY {
    pub fn new(item: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degree_to_radian(angle);
        let mut aabb_box = item.bounding_box(0., 1.).unwrap();
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * aabb_box.max.x + (1 - i) as f64 * aabb_box.min.x;
                    let y = j as f64 * aabb_box.max.y + (1 - j) as f64 * aabb_box.min.y;
                    let z = k as f64 * aabb_box.max.z + (1 - k) as f64 * aabb_box.min.z;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);
                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }
        aabb_box = AABB::new(min, max);

        Self {
            item,
            sin_theta,
            cos_theta,
            aabb_box,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut orig = ray.orig;
        let mut dir = ray.dir;

        orig[0] = self.cos_theta * ray.orig[0] - self.sin_theta * ray.orig[2];
        orig[2] = self.sin_theta * ray.orig[0] + self.cos_theta * ray.orig[2];

        dir[0] = self.cos_theta * ray.dir[0] - self.sin_theta * ray.dir[2];
        dir[2] = self.sin_theta * ray.dir[0] + self.cos_theta * ray.dir[2];

        let rotated_ray = Ray::new(orig, dir, ray.tm);

        if let Some(mut rec) = self.item.hit(&rotated_ray, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            rec.p = p;
            rec.set_face_normal(&rotated_ray, &normal);

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _tm: f64, _dur: f64) -> Option<AABB> {
        Some(self.aabb_box)
    }
}
