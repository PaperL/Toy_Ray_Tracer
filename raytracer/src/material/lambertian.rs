use std::f64::consts::PI;

use crate::{
    basic::{
        ray::Ray,
        vec3::{RGBColor, Vec3},
    },
    hittable::HitRecord,
    material::Material,
    pdf::cos_pdf::CosinePDF,
    texture::{solid_color::SolidColor, Texture},
};

use super::ScatterRecord;

#[derive(Clone)]
pub struct Lambertian<TT>
where
    TT: Texture,
{
    pub albedo: TT,
}

impl<TT: Texture> Lambertian<TT> {
    pub fn new(albedo: TT) -> Self {
        Self { albedo }
    }
}

impl Lambertian<SolidColor> {
    pub fn new_from_color(color_value: RGBColor) -> Self {
        Self {
            albedo: SolidColor::new(color_value),
        }
    }
}

impl<TT: Texture> Material for Lambertian<TT> {
    fn scatter(&self, _ray: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::new_not_specular(
            CosinePDF::new(hit_rec.normal),
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
