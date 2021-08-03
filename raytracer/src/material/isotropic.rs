use crate::{
    basic::{
        ray::Ray,
        vec3::{RGBColor, Vec3},
    },
    hittable::HitRecord,
    pdf::cos_pdf::CosinePDF,
    texture::{solid_color::SolidColor, Texture},
};

use super::{Material, ScatterRecord};

#[derive(Clone)]
pub struct Isotropic<TT>
// isotropic: 各方向同性
where
    TT: Texture,
{
    albedo: TT,
}

impl<TT: Texture> Isotropic<TT> {
    pub fn new(albedo: TT) -> Self {
        Self { albedo }
    }
}

impl Isotropic<SolidColor> {
    pub fn new_from_color(color_value: RGBColor) -> Self {
        Self {
            albedo: SolidColor::new(color_value),
        }
    }
}

impl<TT: Texture> Material for Isotropic<TT> {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::new_not_specular(
            CosinePDF::new(Vec3::rand_unit_sphere()),
            self.albedo.value(rec.u, rec.v, rec.p),
        ))
    }

    fn scattering_pdf(&self, _ray: &Ray, _hit_rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.
    }
}
