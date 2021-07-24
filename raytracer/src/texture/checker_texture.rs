use std::rc::Rc;

use super::Texture;

pub struct CheckerTexture {
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: crate::basic::vec3::Point3) -> crate::basic::vec3::RGBColor {
        let sines = f64::sin(10. * p.x) * f64::sin(10. * p.y) * f64::sin(10. * p.z);
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
