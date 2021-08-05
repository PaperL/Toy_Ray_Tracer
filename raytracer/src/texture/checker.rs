use crate::basic::vec3::{Point3, RGBColor};

use super::Texture;

pub struct Checker<TT1, TT2>
where
    TT1: Texture,
    TT2: Texture,
{
    // 棋盘状纹理, 相邻格子材质为 odd & even
    pub odd: TT1,
    pub even: TT2,
    pub scale: f64,
}

impl<TT1: Texture, TT2: Texture> Checker<TT1, TT2> {
    pub fn new(odd: TT1, even: TT2, scale: f64) -> Self {
        Self { odd, even, scale }
    }
}

impl<TT1: Texture, TT2: Texture> Texture for Checker<TT1, TT2> {
    fn value(&self, u: f64, v: f64, p: Point3) -> RGBColor {
        // shader networks introduced by Pat Hanrahan
        let sines =
            f64::sin(self.scale * p.x) * f64::sin(self.scale * p.y) * f64::sin(self.scale * p.z);
        // println!("{} {}", sines, p);

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
