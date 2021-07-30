use std::f64::consts::PI;

use crate::basic::{onb::ONB, vec3::Vec3};

use super::PDF;

pub struct CosinePDF {
    pub uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: Vec3) -> Self {
        Self {
            uvw: ONB::build_from_w(&w),
        }
    }
}

impl PDF for CosinePDF {
    fn value(&self, dir: &Vec3) -> f64 {
        let cos = Vec3::dot(&dir.to_unit(), &self.uvw.w());
        if cos < 0. {
            0.
        } else {
            cos / PI
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local(&Vec3::rand_cos_dir())
    }
}
