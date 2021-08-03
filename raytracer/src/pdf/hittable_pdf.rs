use crate::{
    basic::vec3::{Point3, Vec3},
    hittable::Hittable,
};

use super::PDF;

pub struct HittablePDF<'a, TH>
where
    TH: Hittable,
{
    orig: Point3,
    obj: &'a TH,
}

impl<'a, TH: Hittable> HittablePDF<'a, TH> {
    pub fn new(orig: Point3, obj: &'a TH) -> Self {
        Self { orig, obj }
    }
}

impl<TH: Hittable> PDF for HittablePDF<'_, TH> {
    fn value(&self, dir: &Vec3) -> f64 {
        self.obj.pdf_value(&self.orig, dir)
    }

    fn generate(&self) -> Vec3 {
        self.obj.rand_dir(&self.orig)
    }
}
