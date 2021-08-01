pub mod instance;
pub mod object;

use std::{f64::consts::PI, sync::Arc};

use rand::prelude::SliceRandom;

use crate::{
    basic::{
        ray::Ray,
        vec3::{Point3, Vec3},
    },
    bvh::aabb::AABB,
    material::Material,
};

//=================================================

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self, tm: f64, dur: f64) -> Option<AABB>;

    fn pdf_value(&self, _orig: &Point3, _dir: &Vec3) -> f64 {
        0.
    }

    fn rand_dir(&self, _orig: &Vec3) -> Vec3 {
        Vec3::new(1., 0., 0.)
    }

    fn map_to(value: f64, width: f64, threshold: f64) -> f64
    where
        Self: Sized,
    {
        f64::atan(value / width) * threshold * 2. / PI
    }
}

//=================================================

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,              // 碰撞点
    pub normal: Vec3,           // 外侧法向量
    pub mat: Arc<dyn Material>, // 材质
    pub t: f64,                 // 碰撞点对应 Ray::at(t)
    pub front_face: bool,       // 光线是否来自外侧
    pub u: f64,                 // 碰撞点对应物体的 u,v, 用于计算贴图
    pub v: f64,                 //
}

impl HitRecord {
    fn new(
        u: f64,
        v: f64,
        t: f64,
        ray: &Ray,
        outward_normal: &Vec3,
        mat: Arc<dyn Material>,
    ) -> Self {
        let mut tmp_rec = Self {
            p: ray.at(t),
            normal: Vec3::default(),
            mat,
            t,
            front_face: bool::default(),
            u,
            v,
        };
        tmp_rec.set_face_normal(ray, outward_normal);

        tmp_rec
    }

    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        // 初始化 front_fase & normal
        self.front_face = Vec3::dot(&ray.dir, &*outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

//=================================================

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
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

    fn pdf_value(&self, orig: &Point3, dir: &Vec3) -> f64 {
        let weight = 1. / self.objects.len() as f64;
        let mut sum = 0.;

        for obj in &self.objects {
            sum += obj.pdf_value(orig, dir) * weight;
        }

        sum
    }

    fn rand_dir(&self, orig: &Vec3) -> Vec3 {
        self.objects
            .choose(&mut rand::thread_rng())
            .unwrap()
            .rand_dir(orig)
    }
}
