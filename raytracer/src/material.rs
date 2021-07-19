use crate::{
    basic::clamp_oi,
    hittable::HitRecord,
    ray::Ray,
    vec3::{RGBColor, Vec3},
};

//====================================================

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut RGBColor) -> Option<Ray>;
}

//====================================================

pub struct Lambertian {
    pub albedo: RGBColor,
}

impl Lambertian {
    pub fn new(albedo: RGBColor) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut RGBColor) -> Option<Ray> {
        let mut scatter_dir = rec.normal + Vec3::rand_unit();
        if scatter_dir.is_zero() {
            scatter_dir = rec.normal;
        }
        *attenuation = self.albedo;

        Some(Ray::new(rec.p, scatter_dir))
    }
}

//====================================================

pub struct Metal {
    pub albedo: RGBColor,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(a: &RGBColor, f: f64) -> Self {
        Self {
            albedo: *a,
            fuzz: clamp_oi(f, 0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut RGBColor) -> Option<Ray> {
        let reflected = Vec3::reflect(&r_in.dir.unit_vector(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + Vec3::rand_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo;

        return if Vec3::dot(&scattered.dir, &rec.normal) > 0.0 {
            Some(scattered)
        } else {
            None
        };
    }
}
