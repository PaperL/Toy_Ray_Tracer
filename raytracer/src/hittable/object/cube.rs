use std::sync::Arc;

use super::{
    super::{HitRecord, Hittable, HittableList},
    rectangle::Rectangle,
};

use crate::{
    basic::{ray::Ray, vec3::Point3},
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
        tmp_cube.sides.add(Rectangle::new(
            0,
            cube_min.x,
            cube_max.x,
            cube_min.y,
            cube_max.y,
            cube_min.z,
            mat.clone(),
        ));
        tmp_cube.sides.add(Rectangle::new(
            0,
            cube_min.x,
            cube_max.x,
            cube_min.y,
            cube_max.y,
            cube_max.z,
            mat.clone(),
        ));
        tmp_cube.sides.add(Rectangle::new(
            1,
            cube_min.y,
            cube_max.y,
            cube_min.z,
            cube_max.z,
            cube_min.x,
            mat.clone(),
        ));
        tmp_cube.sides.add(Rectangle::new(
            1,
            cube_min.y,
            cube_max.y,
            cube_min.z,
            cube_max.z,
            cube_max.x,
            mat.clone(),
        ));
        tmp_cube.sides.add(Rectangle::new(
            2,
            cube_min.x,
            cube_max.x,
            cube_min.z,
            cube_max.z,
            cube_min.y,
            mat.clone(),
        ));
        tmp_cube.sides.add(Rectangle::new(
            2, cube_min.x, cube_max.x, cube_min.z, cube_max.z, cube_max.y, mat,
        ));

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
}
