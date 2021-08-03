use rand::prelude::SliceRandom;

use super::{
    super::{HitRecord, Hittable},
    rectangle::Rectangle,
};

use crate::{
    basic::{
        ray::Ray,
        vec3::{Point3, Vec3},
    },
    bvh::aabb::AABB,
    material::Material,
};

#[derive(Clone)]
pub struct Cube<TM>
where
    TM: Material,
{
    pub min: Point3,
    pub max: Point3,
    pub sides: Vec<Rectangle<TM>>,
}

impl<TM: Material + Clone> Cube<TM> {
    pub fn new(min: Point3, max: Point3, mat: TM) -> Self {
        let mut tmp_cube = Self {
            min,
            max,
            sides: Vec::<Rectangle<TM>>::new(),
        };

        //todo 可用循环减少代码量
        tmp_cube.sides.push(Rectangle::new(
            0,
            min.x,
            max.x,
            min.y,
            max.y,
            min.z,
            mat.clone(),
            false,
        ));
        tmp_cube.sides.push(Rectangle::new(
            0,
            min.x,
            max.x,
            min.y,
            max.y,
            max.z,
            mat.clone(),
            true,
        ));
        tmp_cube.sides.push(Rectangle::new(
            1,
            min.y,
            max.y,
            min.z,
            max.z,
            min.x,
            mat.clone(),
            false,
        ));
        tmp_cube.sides.push(Rectangle::new(
            1,
            min.y,
            max.y,
            min.z,
            max.z,
            max.x,
            mat.clone(),
            true,
        ));
        tmp_cube.sides.push(Rectangle::new(
            2,
            min.x,
            max.x,
            min.z,
            max.z,
            min.y,
            mat.clone(),
            false,
        ));
        tmp_cube.sides.push(Rectangle::new(
            2, min.x, max.x, min.z, max.z, max.y, mat, true,
        ));

        tmp_cube
    }
}

impl<TM: Material> Hittable for Cube<TM> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_rec = None;
        let mut closest_so_far = t_max;

        for i in &self.sides {
            if let Some(temp_hit_rec) = i.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_hit_rec.t;
                hit_rec = Some(temp_hit_rec);
            }
        }

        hit_rec
    }

    fn bounding_box(&self, _tm: f64, _dur: f64) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }

    fn pdf_value(&self, orig: &Point3, dir: &Vec3) -> f64 {
        let mut sum = 0.;
        for obj in &self.sides {
            sum += 1. / obj.pdf_value(orig, dir);
            // Rectangle 的 pdf_value 为长方形在 orig 视野中的面积占比的倒数
        }
        // 长方体六个面的 pdf_value 倒数合为 长方体在 orig 视野中的面积的两倍的占比
        2. / sum
    }

    fn rand_dir(&self, orig: &Point3) -> Vec3 {
        self.sides
            .choose(&mut rand::thread_rng())
            .unwrap()
            .rand_dir(orig)
    }
}
