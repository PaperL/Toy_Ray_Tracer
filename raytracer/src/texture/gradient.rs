use std::f64::consts::PI;

use crate::basic::vec3::{Point3, RGBColor};

use super::Texture;

#[derive(Clone)]
pub struct Gradient {
    pub color_set: Vec<RGBColor>,
    pub pos_set: Vec<f64>,

    color_sub: Vec<RGBColor>,
    pos_sub: Vec<f64>,
    last_n: usize,
}

impl Gradient {
    pub fn new(color_set: Vec<RGBColor>, pos_set: Vec<f64>) -> Self {
        if pos_set.len() < 2 || color_set.len() != pos_set.len() {
            panic!("Get wrong input in texture::Gradient");
        }

        let last_n = pos_set.len() - 1;
        let mut color_sub = Vec::<RGBColor>::new();
        for i in 0..last_n {
            color_sub.push(color_set[i + 1] - color_set[i]);
        }
        let mut pos_sub = Vec::<f64>::new();
        for i in 0..last_n {
            pos_sub.push(pos_set[i + 1] - pos_set[i]);
        }

        Self {
            color_set,
            pos_set,
            color_sub,
            pos_sub,
            last_n,
        }
    }
}

impl Texture for Gradient {
    fn value(&self, _u: f64, v: f64, _p: Point3) -> RGBColor {
        // 正常来说如果 u,v 在 xy 平面上, 那么渐变通常随 x 变化。
        // 但是考虑到本程序中渐变用于球体球坐标, 故以 v 为自变量
        let mut col_id = 1;
        let k = (-f64::cos(v * PI) + 1.) / 2.;
        while col_id < self.last_n && k > self.pos_set[col_id] {
            col_id += 1;
        }
        col_id -= 1;

        self.color_set[col_id]
            + self.color_sub[col_id] * ((k - self.pos_set[col_id]) / self.pos_sub[col_id])
    }
}

#[cfg(test)]
mod tests {
    use crate::basic::vec3::Vec3;

    use super::*;

    #[test]
    fn test() {
        let g = Gradient::new(
            vec![
                RGBColor::new(1., 0., 0.),
                RGBColor::new(0., 1., 0.),
                RGBColor::new(0., 0., 1.),
            ],
            vec![0., 0.5, 1.],
        );
        for i in 0..11 {
            println!("{}", g.value(0., 0.1 * i as f64, Vec3::default()));
        }
    }
}
