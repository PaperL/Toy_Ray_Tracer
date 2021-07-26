use std::rc::Rc;

use crate::{
    basic::{
        ray::Ray,
        vec3::{RGBColor, Vec3},
    },
    texture::{solid_color::SolidColor, Texture},
};

use super::Material;

pub struct Isotropic {
    albedo: Rc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn new_from_color(c: RGBColor) -> Self {
        Self {
            albedo: Rc::new(SolidColor::new(c)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &crate::basic::ray::Ray,
        rec: &crate::hittable::HitRecord,
    ) -> Option<(crate::basic::ray::Ray, RGBColor)> {
        Some((
            Ray::new(rec.p, Vec3::rand_in_unit_sphere(), r_in.tm),
            self.albedo.value(rec.u, rec.v, rec.p),
        ))
    }
}
