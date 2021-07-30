use std::{f64::consts::PI, sync::Arc};

use crate::{
    basic::{
        ray::Ray,
        tp,
        vec3::{RGBColor, Vec3},
    },
    hittable::HitRecord,
    material::Material,
    pdf::cos_pdf::CosinePDF,
    texture::{solid_color::SolidColor, Texture},
};

use super::ScatterRecord;

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
    fn scatter(&self, _ray: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::new_not_specular(
            tp(CosinePDF::new(hit_rec.normal)),
            self.albedo.value(hit_rec.u, hit_rec.v, hit_rec.p),
        ))
    }

    fn scattering_pdf(&self, _ray: &Ray, hit_rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = Vec3::dot(&hit_rec.normal, &scattered.dir.to_unit());
        if cosine.is_sign_negative() {
            0.
        } else {
            cosine / PI
        }
    }
}
