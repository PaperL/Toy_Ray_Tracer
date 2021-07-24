pub mod solid_color;
pub mod checker_texture;

use crate::basic::vec3::{Point3, RGBColor};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> RGBColor;
}
