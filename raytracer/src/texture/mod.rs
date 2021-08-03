// pub mod checker_texture;
pub mod image_texture;
pub mod solid_color;

use crate::basic::vec3::{Point3, RGBColor};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> RGBColor;
}
