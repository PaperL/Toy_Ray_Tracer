use std::sync::Arc;

use crate::basic::vec3::{Point3, RGBColor};

use super::Texture;

pub struct CheckerTexture {
    // 棋盘状纹理, 相邻格子材质为 odd & even
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> RGBColor {
        // shader networks introduced by Pat Hanrahan
        let sines = f64::sin(10. * p.x) * f64::sin(10. * p.y) * f64::sin(10. * p.z);

        // 棒棒糖
        // let odevity = (u * 16. + v * 16.) as i32;
        // 简单网格
        // let odevity = (u * 16.) as i32 + (v * 16.) as i32;

        if sines > 0. {
            // if odevity % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
