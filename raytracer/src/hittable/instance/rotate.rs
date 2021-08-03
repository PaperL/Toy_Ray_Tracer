use std::f64::{INFINITY, NEG_INFINITY};

use super::super::{HitRecord, Hittable};

use crate::{
    basic::{
        degree_to_radian,
        ray::Ray,
        vec3::{Point3, Vec3},
    },
    bvh::aabb::AABB,
};

#[derive(Clone)]
pub struct Rotate<TH>
where
    TH: Hittable,
{
    dir: u32,
    pub obj: TH,
    sin_theta: f64,
    cos_theta: f64,
    pub aabb_box: AABB,
    dio: [usize; 3], // dimension order
}

impl<TH: Hittable> Rotate<TH> {
    pub fn new(obj: TH, dir: u32, angle: f64) -> Self {
        let dio = match dir {
            0 => [2, 1, 0], // x
            1 => [0, 2, 1], // y
            2 => [1, 0, 2], // z
            _ => panic!("Get unexpected dir in Rotate::new!"),
        };

        let radians = degree_to_radian(angle);
        let tmp_box = obj.bounding_box(0., 1.).unwrap();
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let co = [
                        i as f64 * tmp_box.max.x + (1 - i) as f64 * tmp_box.min.x,
                        j as f64 * tmp_box.max.y + (1 - j) as f64 * tmp_box.min.y,
                        k as f64 * tmp_box.max.z + (1 - k) as f64 * tmp_box.min.z,
                    ];

                    let mut new_co = [0.; 3];
                    new_co[dio[0]] = cos_theta * co[dio[0]] + sin_theta * co[dio[1]];
                    new_co[dio[1]] = -sin_theta * co[dio[0]] + cos_theta * co[dio[1]];
                    new_co[dio[2]] = co[dio[2]];

                    let tester = Vec3::new(new_co[0], new_co[1], new_co[2]);
                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }
        let aabb_box = AABB::new(min, max);

        Self {
            dir,
            obj,
            sin_theta,
            cos_theta,
            aabb_box,
            dio,
        }
    }

    fn rotated_orig(&self, orig: &Point3) -> Point3 {
        let mut r_orig = *orig;

        r_orig[self.dio[0]] =
            self.cos_theta * orig[self.dio[0]] - self.sin_theta * orig[self.dio[1]];
        r_orig[self.dio[1]] =
            self.sin_theta * orig[self.dio[0]] + self.cos_theta * orig[self.dio[1]];

        r_orig
    }

    fn rotated_dir(&self, dir: &Vec3) -> Vec3 {
        let mut r_dir = *dir;

        r_dir[self.dio[0]] = self.cos_theta * dir[self.dio[0]] - self.sin_theta * dir[self.dio[1]];
        r_dir[self.dio[1]] = self.sin_theta * dir[self.dio[0]] + self.cos_theta * dir[self.dio[1]];

        r_dir
    }
}

impl<TH: Hittable> Hittable for Rotate<TH> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let orig = self.rotated_orig(&ray.orig);
        let dir = self.rotated_dir(&ray.dir);

        let rotated_ray = Ray::new(orig, dir, ray.tm);

        if let Some(mut rec) = self.obj.hit(&rotated_ray, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[self.dio[0]] =
                self.cos_theta * rec.p[self.dio[0]] + self.sin_theta * rec.p[self.dio[1]];
            p[self.dio[1]] =
                -self.sin_theta * rec.p[self.dio[0]] + self.cos_theta * rec.p[self.dio[1]];

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

    fn pdf_value(&self, orig: &Point3, dir: &Vec3) -> f64 {
        let obj_orig = self.rotated_orig(orig);
        let obj_dir = self.rotated_dir(dir);

        self.obj.pdf_value(&obj_orig, &obj_dir)
    }

    fn rand_dir(&self, orig: &Vec3) -> Vec3 {
        let obj_orig = self.rotated_orig(orig);

        self.obj.rand_dir(&obj_orig)
    }
}
