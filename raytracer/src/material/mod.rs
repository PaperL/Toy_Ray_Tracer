// pub mod dielectric;
pub mod diffuse_light;
// pub mod isotropic;
pub mod lambertian;
// pub mod metal;

use crate::{
    basic::{
        ray::Ray,
        vec3::{Point3, RGBColor},
    },
    hittable::HitRecord,
};

pub trait Material: Send + Sync {
    fn scatter(&self, _ray: &Ray, _rec: &HitRecord) -> Option<(RGBColor, Ray, f64)> {
        None
    }

    fn scattering_pdf(&self, _ray: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.
    }

    fn emitted(&self, _ray: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: Point3) -> RGBColor {
        RGBColor::default()
    }
}
