use std::f64::consts::PI;

use crate::{
    basic::{
        clamp_oi, rand_1,
        ray::Ray,
        vec3::{RGBColor, Vec3},
    },
    hittable::HitRecord,
    material::Material,
    pdf::cos_pdf::CosinePDF,
    texture::Texture,
};

use super::ScatterRecord;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct ColoredMetal<TT>
where
    TT: Texture,
{
    pub albedo: RGBColor, // 反射率
    pub fuzz: f64,        // fuzziness, 模糊
    pub tex: TT,          // 材质
}

impl<TT: Texture> ColoredMetal<TT> {
    pub fn new(albedo: RGBColor, f: f64, tex: TT) -> Self {
        Self {
            albedo,
            fuzz: clamp_oi(f, 0., 1.),
            tex,
        }
    }
}

impl<TT: Texture> Material for ColoredMetal<TT> {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<ScatterRecord> {
        if rand_1() < 0.25 {
            let reflected = Vec3::reflect(&ray.dir.to_unit(), &hit_rec.normal);

            Some(ScatterRecord::new_specular(
                Ray::new(
                    hit_rec.p,
                    reflected + Vec3::rand_unit_sphere() * self.fuzz,
                    ray.tm,
                ),
                self.albedo,
            ))
        } else {
            Some(ScatterRecord::new_not_specular(
                CosinePDF::new(hit_rec.normal),
                self.tex.value(hit_rec.u, hit_rec.v, hit_rec.p),
            ))
        }
    }

    fn scattering_pdf(&self, _ray: &Ray, hit_rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = Vec3::dot(&hit_rec.normal, &scattered.dir.to_unit());
        if cosine.is_sign_negative() {
            0.
        } else {
            cosine / PI
        }
    }
}
