use std::{f64::consts::PI, sync::Arc};

use crate::{
    basic::{
        onb::ONB,
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
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray, f64)> {
        let uvw = ONB::build_from_w(&rec.normal);
        let dir = uvw.local(&Vec3::rand_cos_dir());
        let scattered = Ray::new(rec.p, dir.to_unit(), ray.tm);
        let albedo = self.albedo.value(rec.u, rec.v, rec.p);
        let pdf = Vec3::dot(&uvw.axis[2], &scattered.dir) / PI;
        // println!("scatter {} {} {}", albedo, scattered.dir, pdf);

        Some((albedo, scattered, pdf))
    }

    fn scattering_pdf(&self, _ray: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = Vec3::dot(&rec.normal, &scattered.dir.to_unit());

        // println!("pdf {}", cosine / PI);

        if cosine.is_sign_negative() {
            0.
        } else {
            cosine / PI
        }
    }
}
