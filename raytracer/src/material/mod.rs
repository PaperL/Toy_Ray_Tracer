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
    pdf::cos_pdf::CosinePDF,
};

pub trait Material: Send + Sync {
    fn emitted(&self, _ray: &Ray, _hit_rec: &HitRecord, _u: f64, _v: f64, _p: Point3) -> RGBColor {
        RGBColor::default()
    }

    fn scatter(&self, _ray: &Ray, _hit_rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn scattering_pdf(&self, _ray: &Ray, _hit_rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.
    }
}

//=================================================

pub struct ScatterRecord {
    pub dat: ScaRecData,
    pub attenutaion: RGBColor,
}

pub enum ScaRecData {
    Specular(Ray),
    Pdf(CosinePDF),
}

impl ScatterRecord {
    pub fn new_specular(ray: Ray, attenutaion: RGBColor) -> Self {
        Self {
            dat: ScaRecData::Specular(ray),
            attenutaion,
        }
    }

    pub fn new_not_specular(pdf: CosinePDF, attenutaion: RGBColor) -> Self {
        Self {
            dat: ScaRecData::Pdf(pdf),
            attenutaion,
        }
    }
}
