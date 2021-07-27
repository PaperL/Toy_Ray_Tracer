pub mod instance;
pub mod object;

use std::sync::Arc;

use crate::{
    basic::{
        ray::Ray,
        vec3::{Point3, Vec3},
    },
    bvh::aabb::AABB,
    material::Material,
};

//=================================================

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

    fn bounding_box(&self, tm: f64, dur: f64) -> Option<AABB>;
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
    pub v: f64,
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
    pub fn add<T>(&mut self, object: T)
    where
        T: Hittable + 'static,
    {
        self.objects.push(Arc::new(object));
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
