pub mod moving_sphere;
pub mod sphere;

use std::rc::Rc;

use crate::{
    basic::ray::Ray,
    basic::vec3::{Point3, Vec3},
    bvh::aabb::AABB,
    material::Material,
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
                closest_so_far = temp_rec.t.clone();
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
