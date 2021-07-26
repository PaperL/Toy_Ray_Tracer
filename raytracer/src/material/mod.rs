pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use crate::{
    basic::{
        ray::Ray,
        vec3::{Point3, RGBColor},
    },
    hittable::HitRecord,
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, RGBColor)>;

    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> RGBColor {
        RGBColor::new(0., 0., 0.)
    }
}
