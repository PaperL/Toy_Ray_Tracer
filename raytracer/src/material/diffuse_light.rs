use std::rc::Rc;

use crate::{
    basic::{
        ray::Ray,
        vec3::{Point3, RGBColor, Vec3},
    },
    texture::{solid_color::SolidColor, Texture},
};

use super::Material;

pub struct DiffuseLight {
    emit: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new_from_color(color_value: RGBColor) -> Self {
        Self {
            emit: Rc::new(SolidColor { color_value }),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &crate::basic::ray::Ray,
        _rec: &crate::hittable::HitRecord,
    ) -> Option<(Ray, Vec3)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point3) -> RGBColor {
        self.emit.value(u, v, p)
    }
}
