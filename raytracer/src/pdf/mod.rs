pub mod cos_pdf;
pub mod hittable_pdf;

use std::sync::Arc;

use crate::basic::{rand_1, vec3::Vec3};

pub trait PDF {
    fn value(&self, dir: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}

//=================================================

pub struct MixedPDF {
    pdf: [Arc<dyn PDF>; 2],
}

impl MixedPDF {
    pub fn new(pdf0: Arc<dyn PDF>, pdf1: Arc<dyn PDF>) -> Self {
        Self { pdf: [pdf0, pdf1] }
    }
}

impl PDF for MixedPDF {
    fn value(&self, dir: &Vec3) -> f64 {
        0.5 * self.pdf[0].value(dir) + 0.5 * self.pdf[1].value(dir)
    }

    fn generate(&self) -> Vec3 {
        if rand_1() < 0.5 {
            self.pdf[0].generate()
        } else {
            self.pdf[1].generate()
        }
    }
}
