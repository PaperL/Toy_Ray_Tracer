use std::sync::Arc;

use rand::prelude::SliceRandom;

use super::{
    super::{HitRecord, Hittable, HittableList},
    rectangle::Rectangle,
};

use crate::{
    basic::{
        ray::Ray,
        tp,
        vec3::{Point3, Vec3},
    },
    bvh::aabb::AABB,
    material::Material,
};

#[derive(Clone)]
pub struct Cube {
    pub cube_min: Point3,
    pub cube_max: Point3,
    pub sides: HittableList,
}

impl Cube {
    pub fn new(cube_min: Point3, cube_max: Point3, mat: Arc<dyn Material>) -> Self {
        let mut tmp_cube = Self {
            cube_min,
            cube_max,
            sides: HittableList::default(),
        };

        //todo 可用循环减少代码量
        tmp_cube.sides.add(tp(Rectangle::new(
            0,
            cube_min.x,
            cube_max.x,
            cube_min.y,
            cube_max.y,
            cube_min.z,
            mat.clone(),
            false,
        )));
        tmp_cube.sides.add(tp(Rectangle::new(
            0,
            cube_min.x,
            cube_max.x,
            cube_min.y,
            cube_max.y,
            cube_max.z,
            mat.clone(),
            true,
        )));
        tmp_cube.sides.add(tp(Rectangle::new(
            1,
            cube_min.y,
            cube_max.y,
            cube_min.z,
            cube_max.z,
            cube_min.x,
            mat.clone(),
            false,
        )));
        tmp_cube.sides.add(tp(Rectangle::new(
            1,
            cube_min.y,
            cube_max.y,
            cube_min.z,
            cube_max.z,
            cube_max.x,
            mat.clone(),
            true,
        )));
        tmp_cube.sides.add(tp(Rectangle::new(
            2,
            cube_min.x,
            cube_max.x,
            cube_min.z,
            cube_max.z,
            cube_min.y,
            mat.clone(),
            false,
        )));
        tmp_cube.sides.add(tp(Rectangle::new(
            2, cube_min.x, cube_max.x, cube_min.z, cube_max.z, cube_max.y, mat, true,
        )));

        tmp_cube
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _tm: f64, _dur: f64) -> Option<AABB> {
        Some(AABB::new(self.cube_min, self.cube_max))
    }

    fn pdf_value(&self, orig: &Point3, dir: &Vec3) -> f64 {
        let mut sum = 0.;
        for item in &self.sides.objects {
            sum += 1. / item.pdf_value(orig, dir);
            // Rectangle 的 pdf_value 为长方形在 orig 视野中的面积占比的倒数
        }
        // 长方体六个面的 pdf_value 倒数合为 长方体在 orig 视野中的面积的两倍的占比
        2. / sum
    }

    fn rand_dir(&self, orig: &Point3) -> Vec3 {
        self.sides
            .objects
            .choose(&mut rand::thread_rng())
            .unwrap()
            .rand_dir(orig)
    }
}
