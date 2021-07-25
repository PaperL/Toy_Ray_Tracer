use std::rc::Rc;

use crate::{basic::vec3::Point3, bvh::aabb::AABB, material::Material};

use super::{rectangle::Rectangle, Hittable, HittableList};

pub struct Cube {
    pub cube_min: Point3,
    pub cube_max: Point3,
    pub sides: HittableList,
}

impl Cube {
    pub fn new(cube_min: Point3, cube_max: Point3, mat_ptr: Rc<dyn Material>) -> Self {
        let mut tmp_cube = Self {
            cube_min,
            cube_max,
            sides: HittableList::default(),
        };

        tmp_cube.sides.add(Rectangle::new(
            0,
            cube_min.x,
            cube_max.x,
            cube_min.y,
            cube_max.y,
            cube_min.z,
            mat_ptr.clone(),
        ));
        tmp_cube.sides.add(Rectangle::new(
            0,
            cube_min.x,
            cube_max.x,
            cube_min.y,
            cube_max.y,
            cube_max.z,
            mat_ptr.clone(),
        ));

        tmp_cube.sides.add(Rectangle::new(
            1,
            cube_min.y,
            cube_max.y,
            cube_min.z,
            cube_max.z,
            cube_min.x,
            mat_ptr.clone(),
        ));
        tmp_cube.sides.add(Rectangle::new(
            1,
            cube_min.y,
            cube_max.y,
            cube_min.z,
            cube_max.z,
            cube_max.x,
            mat_ptr.clone(),
        ));

        tmp_cube.sides.add(Rectangle::new(
            2,
            cube_min.x,
            cube_max.x,
            cube_min.z,
            cube_max.z,
            cube_min.y,
            mat_ptr.clone(),
        ));
        tmp_cube.sides.add(Rectangle::new(
            2, cube_min.x, cube_max.x, cube_min.z, cube_max.z, cube_max.y, mat_ptr,
        ));

        tmp_cube
    }
}

impl Hittable for Cube {
    fn hit(
        &self,
        ray: &crate::basic::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<super::HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _time: f64, _dur: f64) -> Option<crate::bvh::aabb::AABB> {
        Some(AABB::new(self.cube_min, self.cube_max))
    }
}
