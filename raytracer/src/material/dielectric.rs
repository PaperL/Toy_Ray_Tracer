use crate::basic::{
    ray::Ray,
    vec3::{RGBColor, Vec3},
};
use crate::hittable::HitRecord;
use crate::material::Material;
use rand::random;

pub struct Dielectric {
    pub ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn reflectance(cos: f64, ir: f64) -> f64 {
        // Use Schlink's approximation for reflectance.
        let r0 = ((1. - ir) / (1. + ir)).powi(2);
        r0 + (1. - r0) * (1. - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let refraction_ratio = if rec.front_face {
            1. / self.ir
        } else {
            self.ir
        };

        let unit_dir = r_in.dir.unit_vector();
        let cos_theta = f64::min(Vec3::dot(&-unit_dir, &rec.normal), 1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let dir: Vec3;
        if refraction_ratio * sin_theta > 1.   // Cannot Refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random::<f64>()
        {
            dir = Vec3::reflect(&unit_dir, &rec.normal);
        } else {
            dir = Vec3::refract(&unit_dir, &rec.normal, refraction_ratio);
        }

        Some((Ray::new(rec.p, dir, r_in.tm), RGBColor::new(1., 1., 1.)))
    }
}
