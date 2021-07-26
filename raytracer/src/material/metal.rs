use crate::basic::{
    clamp_oi,
    ray::Ray,
    vec3::{RGBColor, Vec3},
};
use crate::hittable::HitRecord;
use crate::material::Material;

pub struct Metal {
    pub albedo: RGBColor,
    pub fuzz: f64,
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = Vec3::reflect(&r_in.dir.unit_vector(), &rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + Vec3::rand_in_unit_sphere() * self.fuzz,
            r_in.tm,
        );

        if Vec3::dot(&scattered.dir, &rec.normal) > 0. {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
