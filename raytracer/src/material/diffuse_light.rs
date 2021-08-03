use crate::{
    basic::{
        ray::Ray,
        vec3::{Point3, RGBColor},
    },
    hittable::HitRecord,
    texture::{solid_color::SolidColor, Texture},
};

use super::Material;

#[derive(Clone)]
pub struct DiffuseLight<TT>
where
    TT: Texture,
{
    emit: TT,
}

impl<TT: Texture> DiffuseLight<TT> {
    pub fn new(emit: TT) -> Self {
        Self { emit }
    }
}

impl DiffuseLight<SolidColor> {
    pub fn new_from_color(color_value: RGBColor) -> Self {
        Self {
            emit: SolidColor::new(color_value),
        }
    }
}

impl<TT: Texture> Material for DiffuseLight<TT> {
    fn emitted(&self, _ray: &Ray, _rec: &HitRecord, u: f64, v: f64, p: Point3) -> RGBColor {
        self.emit.value(u, v, p)
        // if rec.front_face {
        //     self.emit.value(u, v, p)
        // } else {
        //     Vec3::default()
        // }
    }
}
