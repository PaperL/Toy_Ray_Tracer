use crate::basic::{
    ray::Ray,
    vec3::{RGBColor, Vec3},
};
use crate::hittable::HitRecord;
use crate::material::Material;

pub struct Lambertian {
    pub albedo: RGBColor,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut RGBColor) -> Option<Ray> {
        let mut scatter_dir = rec.normal + Vec3::rand_unit();
        if scatter_dir.is_zero() {
            scatter_dir = rec.normal;
        }
        *attenuation = self.albedo;

        Some(Ray::new(rec.p, scatter_dir, r_in.tm))
    }
}
