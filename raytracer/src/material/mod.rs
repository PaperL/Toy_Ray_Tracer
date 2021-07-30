// pub mod dielectric;
pub mod diffuse_light;
// pub mod isotropic;
pub mod lambertian;
// pub mod metal;

use std::sync::Arc;

use crate::{
    basic::{
        ray::Ray,
        vec3::{Point3, RGBColor},
    },
    hittable::HitRecord,
    pdf::PDF,
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

pub struct ScatterRecord {
    // 如果是镜面反射 (Specular), 则没有 PDF
    pub specular: Option<Ray>,
    pub pdf: Option<Arc<dyn PDF>>,
    pub attenutaion: RGBColor,
}

impl ScatterRecord {
    pub fn new_specular(specular: Ray, attenutaion: RGBColor) -> Self {
        Self {
            specular: Some(specular),
            pdf: None,
            attenutaion,
        }
    }

    pub fn new_not_specular(pdf: Arc<dyn PDF>, attenutaion: RGBColor) -> Self {
        Self {
            specular: None,
            pdf: Some(pdf),
            attenutaion,
        }
    }
}
