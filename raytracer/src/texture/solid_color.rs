use crate::basic::vec3::RGBColor;

use super::Texture;

pub struct SolidColor {
    pub color_value: RGBColor,
}

impl SolidColor {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            color_value: RGBColor::new(x, y, z),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: crate::basic::vec3::Point3) -> RGBColor {
        self.color_value
    }
}
