use std::sync::Arc;

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
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn new_from_color(color_value: RGBColor) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(color_value)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_dir = rec.normal + Vec3::rand_unit();
        if scatter_dir.is_zero() {
            scatter_dir = rec.normal;
        }

        Some((
            Ray::new(rec.p, scatter_dir, ray.tm),
            self.albedo.value(rec.u, rec.v, rec.p),
        ))
    }
}
