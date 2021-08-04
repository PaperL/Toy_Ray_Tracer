pub mod cos_pdf;
pub mod hittable_pdf;

use crate::{
    basic::{rand_1, vec3::Vec3},
    hittable::Hittable,
};

use self::{cos_pdf::CosinePDF, hittable_pdf::HittablePDF};

pub trait PDF {
    fn value(&self, dir: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}

//=================================================

pub struct MixedPDF<'a, TH>
where
    TH: Hittable,
{
    scatter_pdf: CosinePDF,
    light_pdf: HittablePDF<'a, TH>,
}

impl<'a, TH: Hittable> MixedPDF<'a, TH> {
    pub fn new(scatter_pdf: CosinePDF, light_pdf: HittablePDF<'a, TH>) -> Self {
        Self {
            scatter_pdf,
            light_pdf,
        }
    }
}

impl<'a, TH: Hittable> PDF for MixedPDF<'a, TH> {
    fn value(&self, dir: &Vec3) -> f64 {
        self.scatter_pdf.value(dir) + self.light_pdf.value(dir)
    }

    fn generate(&self) -> Vec3 {
        if rand_1() < 0.7 {
            self.scatter_pdf.generate()
        } else {
            self.light_pdf.generate()
        }
    }
}
