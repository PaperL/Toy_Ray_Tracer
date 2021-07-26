pub mod constant_medium;
pub mod cube;
pub mod moving_sphere;
pub mod rectangle;
pub mod sphere;

use std::f64::{INFINITY, NEG_INFINITY};

use crate::basic::degree_to_radian;

use {
    crate::{
        basic::ray::Ray,
        basic::vec3::{Point3, Vec3},
        bvh::aabb::AABB,
        material::Material,
    },
    std::rc::Rc,
};

//=================================================

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self, time: f64, dur: f64) -> Option<AABB>;
}

//=================================================

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    fn new(
        u: f64,
        v: f64,
        t: f64,
        ray: &Ray,
        outward_normal: &Vec3,
        mat_ptr: Rc<dyn Material>,
    ) -> Self {
        let mut tmp_rec = Self {
            p: ray.at(t),
            normal: Vec3::default(),
            mat_ptr,
            t,
            front_face: bool::default(),
            u,
            v,
        };
        tmp_rec.set_face_normal(ray, outward_normal);

        tmp_rec
    }

    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&ray.dir, &*outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

//=================================================

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    // pub fn add(&mut self, object: &'a (dyn Hittable + 'a))
    pub fn add<T>(&mut self, object: T)
    where
        T: Hittable + 'static,
    {
        self.objects.push(Rc::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec = None;
        let mut closest_so_far = t_max;

        for i in &self.objects {
            if let Some(temp_rec) = i.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }
        rec
    }

    fn bounding_box(&self, time: f64, dur: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        };

        let mut tot_box = AABB::default();
        let mut first_flag = true;
        for item in &self.objects {
            if let Some(tmp_box) = item.bounding_box(time, dur) {
                if !first_flag {
                    tot_box = AABB::surrounding_box(&tot_box, &tmp_box);
                } else {
                    tot_box = tmp_box;
                    first_flag = false;
                }
            }
        }
        Some(tot_box)
    }
}

//=================================================

pub struct Translate {
    pub hit_ptr: Rc<dyn Hittable>,
    pub offset: Vec3,
}

impl Translate {
    pub fn new(hit_ptr: Rc<dyn Hittable>, offset: Vec3) -> Self {
        Self { hit_ptr, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.orig - self.offset, ray.dir, ray.tm);
        if let Some(mut rec) = self.hit_ptr.hit(&moved_ray, t_min, t_max) {
            rec.p += self.offset;
            rec.set_face_normal(&moved_ray, &rec.normal.clone());
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, time: f64, dur: f64) -> Option<AABB> {
        if let Some(output_box) = self.hit_ptr.bounding_box(time, dur) {
            Some(AABB::new(
                output_box.min + self.offset,
                output_box.max + self.offset,
            ))
        } else {
            None
        }
    }
}

//=================================================

pub struct RotateY {
    hit_ptr: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    aabb_box: Option<AABB>,
}

impl RotateY {
    pub fn new(hit_ptr: Rc<dyn Hittable>, angle: f64) -> Self {
        let radians = degree_to_radian(angle);
        let mut aabb_box = hit_ptr.bounding_box(0., 1.);
        let sin_theta = f64::sin(radians);
        let cos_theta = f64::cos(radians);

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * aabb_box.unwrap().max.x
                        + (1 - i) as f64 * aabb_box.unwrap().min.x;
                    let y = j as f64 * aabb_box.unwrap().max.y
                        + (1 - j) as f64 * aabb_box.unwrap().min.y;
                    let z = k as f64 * aabb_box.unwrap().max.z
                        + (1 - k) as f64 * aabb_box.unwrap().min.z;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);
                    for c in 0..2 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }
        aabb_box = Some(AABB::new(min, max));

        Self {
            hit_ptr,
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

        if let Some(mut rec) = self.hit_ptr.hit(&rotated_ray, t_min, t_max) {
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

    fn bounding_box(&self, _time: f64, _dur: f64) -> Option<AABB> {
        self.aabb_box
    }
}
