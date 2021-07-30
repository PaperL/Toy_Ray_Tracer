use crate::{
    basic::{
        clamp_oi,
        ray::Ray,
        vec3::{RGBColor, Vec3},
    },
    hittable::HitRecord,
    material::Material,
};

use super::ScatterRecord;

pub struct Metal {
    pub albedo: RGBColor, // 反射率
    pub fuzz: f64,        // fuzziness, 模糊
}

impl Metal {
    pub fn new(albedo: RGBColor, f: f64) -> Self {
        Self {
            albedo,
            fuzz: clamp_oi(f, 0., 1.),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = Vec3::reflect(&ray.dir.to_unit(), &rec.normal);

        Some(ScatterRecord::new_specular(
            Ray::new(
                rec.p,
                reflected + Vec3::rand_unit_sphere() * self.fuzz,
                ray.tm,
            ),
            self.albedo,
        ))
    }
}
