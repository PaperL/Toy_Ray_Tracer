use std::rc::Rc;

use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

//=================================================

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&ray.dir, &*outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

//=================================================

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
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
}

//=================================================

#[derive(Clone)]
pub struct Sphere {
    pub cen: Point3, // center
    pub r: f64,      // r
    pub mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, mat_ptr: Rc<dyn Material>) -> Self {
        Self { cen, r, mat_ptr }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.orig - self.cen;
        let a = ray.dir.length_squared();
        let half_b = Vec3::dot(&oc, &ray.dir);
        let c = oc.length_squared() - self.r.powi(2);

        let discriminant = half_b.powi(2) - (a * c);
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let mut rec = HitRecord {
            p: ray.at(root),
            normal: Vec3::default(),
            mat_ptr: self.mat_ptr.clone(),
            t: root,
            front_face: bool::default(),
        };
        // rec.t = root;
        // rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.cen) / self.r;
        rec.set_face_normal(ray, &outward_normal);

        Some(rec)
    }
}
