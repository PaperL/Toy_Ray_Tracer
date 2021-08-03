use std::f64::{consts::PI, INFINITY, NEG_INFINITY};

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
pub struct MotionRotate<TH>
where
    TH: Hittable,
{
    pub dir: u32,
    pub angle: f64,
    pub obj: TH,
    dio: [usize; 3], // dimension order

    pub tm: f64,  // 出现 & 运动开始时间
    pub dur: f64, // 消失 & 运动停止时间
}

impl<TH: Hittable> MotionRotate<TH> {
    pub fn new(obj: TH, dir: u32, angle: f64, tm: f64, dur: f64) -> Self {
        let dio = match dir {
            0 => [2, 1, 0], // x
            1 => [0, 2, 1], // y
            2 => [1, 0, 2], // z
            _ => panic!("Get unexpected dir in MotionRotate::new!"),
        };
        Self {
            dir,
            angle,
            obj,
            dio,
            tm,
            dur,
        }
    }

    fn get_trigonometric(&self, time: f64) -> (f64, f64) {
        let radians = degree_to_radian(self.angle * (time - self.tm) / self.dur);
        // (sin_theta, cos_theta)
        (f64::sin(radians), f64::cos(radians))
    }

    fn get_box(&self, sin_theta: f64, cos_theta: f64) -> AABB {
        let tmp_box = self.obj.bounding_box(0., 1.).unwrap();

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
                    new_co[self.dio[0]] = cos_theta * co[self.dio[0]] + sin_theta * co[self.dio[1]];
                    new_co[self.dio[1]] =
                        -sin_theta * co[self.dio[0]] + cos_theta * co[self.dio[1]];
                    new_co[self.dio[2]] = co[self.dio[2]];

                    let tester = Vec3::new(new_co[0], new_co[1], new_co[2]);
                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }
        AABB::new(min, max)
    }

    fn rotated_orig(&self, orig: &Point3, sin_theta: f64, cos_theta: f64) -> Point3 {
        let mut r_orig = *orig;

        r_orig[self.dio[0]] = cos_theta * orig[self.dio[0]] - sin_theta * orig[self.dio[1]];
        r_orig[self.dio[1]] = sin_theta * orig[self.dio[0]] + cos_theta * orig[self.dio[1]];

        r_orig
    }

    fn rotated_dir(&self, dir: &Vec3, sin_theta: f64, cos_theta: f64) -> Vec3 {
        let mut r_dir = *dir;

        r_dir[self.dio[0]] = cos_theta * dir[self.dio[0]] - sin_theta * dir[self.dio[1]];
        r_dir[self.dio[1]] = sin_theta * dir[self.dio[0]] + cos_theta * dir[self.dio[1]];

        r_dir
    }
}

impl<TH: Hittable> Hittable for MotionRotate<TH> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (sin_theta, cos_theta) = self.get_trigonometric(ray.tm);
        let orig = self.rotated_orig(&ray.orig, sin_theta, cos_theta);
        let dir = self.rotated_dir(&ray.dir, sin_theta, cos_theta);

        let rotated_ray = Ray::new(orig, dir, ray.tm);

        if let Some(mut rec) = self.obj.hit(&rotated_ray, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[self.dio[0]] = cos_theta * rec.p[self.dio[0]] + sin_theta * rec.p[self.dio[1]];
            p[self.dio[1]] = -sin_theta * rec.p[self.dio[0]] + cos_theta * rec.p[self.dio[1]];

            normal[0] = cos_theta * rec.normal[0] + sin_theta * rec.normal[2];
            normal[2] = -sin_theta * rec.normal[0] + cos_theta * rec.normal[2];

            rec.p = p;
            rec.set_face_normal(&rotated_ray, &normal);

            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _tm: f64, _dur: f64) -> Option<AABB> {
        let turn_r = degree_to_radian(self.angle); // radian
        let mut output_box = self.get_box(0., 1.);
        let orig_r = f64::atan(output_box.max[self.dio[1]] / output_box.max[self.dio[0]]);
        let true_r = orig_r + turn_r;

        let mut radians;
        if true_r > PI / 2. {
            radians = PI / 2.;
            output_box = AABB::surrounding_box(
                &output_box,
                &self.get_box(f64::sin(radians), f64::cos(radians)),
            );

            if true_r > PI {
                radians = degree_to_radian(135.);
                output_box = AABB::surrounding_box(
                    &output_box,
                    &self.get_box(f64::sin(radians), f64::cos(radians)),
                );

                if true_r > PI * 1.5 {
                    radians = degree_to_radian(225.);
                    output_box = AABB::surrounding_box(
                        &output_box,
                        &self.get_box(f64::sin(radians), f64::cos(radians)),
                    );
                    // 实际上进入这个分支的情况下, 已经无需算 turn_r 状态的 AABB box
                }
            }
        }
        output_box = AABB::surrounding_box(
            &output_box,
            &self.get_box(f64::sin(turn_r), f64::cos(turn_r)),
        );

        Some(output_box)
    }
}
