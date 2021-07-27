use crate::basic::vec3::{Point3, RGBColor};

use super::Texture;

pub struct SolidColor {
    pub color_value: RGBColor,
}

impl SolidColor {
    pub fn new(color_value: RGBColor) -> Self {
        Self { color_value }
    }

    pub fn new_from_value(r: f64, g: f64, b: f64) -> Self {
        Self {
            color_value: RGBColor::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point3) -> RGBColor {
        self.color_value
    }
}
