use std::sync::Arc;

use crate::{
    basic::{
        ray::Ray,
        vec3::{Point3, RGBColor, Vec3},
    },
    hittable::HitRecord,
    texture::{solid_color::SolidColor, Texture},
};

use super::Material;

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }

    pub fn new_from_color(color_value: RGBColor) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(color_value)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _rec: &HitRecord) -> Option<(Ray, Vec3)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point3) -> RGBColor {
        self.emit.value(u, v, p)
    }
}
