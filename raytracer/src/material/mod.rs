use rand::random;

use super::{
    basic::clamp_oi,
    basic::ray::Ray,
    basic::vec3::{RGBColor, Vec3},
    hittable::HitRecord,
};

//====================================================

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut RGBColor) -> Option<Ray>;
}

//====================================================

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
            fuzz: clamp_oi(f, 0., 1.),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut RGBColor) -> Option<Ray> {
        let reflected = Vec3::reflect(&r_in.dir.unit_vector(), &rec.normal);
        let scattered = Ray::new(rec.p, reflected + Vec3::rand_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo;

        return if Vec3::dot(&scattered.dir, &rec.normal) > 0. {
            Some(scattered)
        } else {
            None
        };
    }
}

//====================================================

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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut RGBColor) -> Option<Ray> {
        *attenuation = RGBColor::new(1., 1., 1.);
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

        Some(Ray::new(rec.p, dir))
    }
}
