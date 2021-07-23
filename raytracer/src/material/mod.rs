pub mod lambertian;
pub mod metal;
pub mod dielectric;

use super::{
    basic::ray::Ray,
    basic::vec3::RGBColor,
    hittable::HitRecord,
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut RGBColor) -> Option<Ray>;
}