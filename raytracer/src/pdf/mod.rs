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
    scatter_pdf: Arc<dyn PDF>,
    light_pdf: Arc<dyn PDF>,
}

impl MixedPDF {
    pub fn new(scatter_pdf: Arc<dyn PDF>, light_pdf: Arc<dyn PDF>) -> Self {
        Self {
            scatter_pdf,
            light_pdf,
        }
    }
}

impl PDF for MixedPDF {
    fn value(&self, dir: &Vec3) -> f64 {
        0.7 * self.scatter_pdf.value(dir) + 0.3 * self.light_pdf.value(dir)
    }

    fn generate(&self) -> Vec3 {
        if rand_1() < 0.65 {
            self.scatter_pdf.generate()
        } else {
            self.light_pdf.generate()
        }
    }
}
