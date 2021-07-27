use super::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub orig: Point3, // origin           光线起点
    pub dir: Vec3,    // direction        光线方向
    pub tm: f64,      // current moment   当前时刻
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, tm: f64) -> Self {
        Self { orig, dir, tm }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
