use std::sync::Arc;

use crate::{
    basic::vec3::{Point3, Vec3},
    hittable::Hittable,
};

use super::PDF;

pub struct HittablePDF {
    orig: Point3,
    item: Arc<dyn Hittable>,
}

impl HittablePDF {
    pub fn new(orig: Point3, item: Arc<dyn Hittable>) -> Self {
        Self { orig, item }
    }
}

impl PDF for HittablePDF {
    fn value(&self, dir: &Vec3) -> f64 {
        self.item.pdf_value(&self.orig, dir)
    }

    fn generate(&self) -> Vec3 {
        self.item.rand_dir(&self.orig)
    }
}
