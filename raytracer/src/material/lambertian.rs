use std::rc::Rc;

use crate::{
    basic::{
        ray::Ray,
        vec3::{RGBColor, Vec3},
    },
    hittable::HitRecord,
    material::Material,
    texture::{solid_color::SolidColor, Texture},
};

pub struct Lambertian {
    pub albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn new_from_color(color_value: RGBColor) -> Self {
        Self {
            albedo: Rc::new(SolidColor { color_value }),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_dir = rec.normal + Vec3::rand_unit();
        if scatter_dir.is_zero() {
            scatter_dir = rec.normal;
        }

        Some((
            Ray::new(rec.p, scatter_dir, r_in.tm),
            self.albedo.value(rec.u, rec.v, rec.p),
        ))
    }
}
