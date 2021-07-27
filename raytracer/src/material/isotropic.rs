use std::sync::Arc;

use crate::{
    basic::{
        ray::Ray,
        vec3::{RGBColor, Vec3},
    },
    hittable::HitRecord,
    texture::{solid_color::SolidColor, Texture},
};

use super::Material;

pub struct Isotropic {
    // isotropic: 各方向同性
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn new_from_color(color_value: RGBColor) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(color_value)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(crate::basic::ray::Ray, RGBColor)> {
        Some((
            Ray::new(rec.p, Vec3::rand_unit_sphere(), ray.tm),
            self.albedo.value(rec.u, rec.v, rec.p),
        ))
    }
}
