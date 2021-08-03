use std::cmp::Ordering;

use rand::Rng;

use super::aabb::AABB;
use crate::{
    basic::ray::Ray,
    hittable::{HitRecord, Hittable, HittableList},
};

pub struct BvhNode {
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
    aabb_box: AABB,
}

impl BvhNode {
    fn new(
        left: Option<Box<dyn Hittable>>,
        right: Option<Box<dyn Hittable>>,
        tm: f64,
        dur: f64,
    ) -> Self {
        if left.is_none() {
            panic!("BvhNode get null left child!");
        }
        let box_left = left.as_ref().unwrap().bounding_box(tm, dur).unwrap();
        if right.is_some() {
            let box_right = right.as_ref().unwrap().bounding_box(tm, dur).unwrap();
            Self {
                left,
                right,
                aabb_box: AABB::surrounding_box(&box_left, &box_right),
            }
        } else {
            Self {
                left,
                right,
                aabb_box: box_left,
            }
        }
    }

    pub fn new_from_list(hittable_list: HittableList, tm: f64, dur: f64) -> Self {
        Self::new_from_vec(hittable_list.objects, tm, dur)
    }

    pub fn new_from_vec(mut objects: Vec<Box<dyn Hittable>>, tm: f64, dur: f64) -> Self {
        let mut rnd = rand::thread_rng();
        let axis = rnd.gen_range(0..3);
        let comparator = |x: &Box<dyn Hittable>, y: &Box<dyn Hittable>| {
            f64::partial_cmp(
                &(x.bounding_box(tm, dur).unwrap().min[axis]),
                &(y.bounding_box(tm, dur).unwrap().min[axis]),
            )
            .unwrap()
        };

        let object_span = objects.len();
        if object_span == 0 {
            panic!("Get empty Vec at BvhNode::new_from_vec!");
        } else if object_span == 1 {
            let obj = objects.pop().unwrap();
            Self::new(Some(obj), None, tm, dur)
        } else if object_span == 2 {
            let obj0 = objects.pop().unwrap();
            let obj1 = objects.pop().unwrap();
            match comparator(&obj0, &obj1) {
                Ordering::Less => Self::new(Some(obj0), Some(obj1), tm, dur),
                _ => Self::new(Some(obj1), Some(obj0), tm, dur),
            }
        } else {
            objects.sort_unstable_by(comparator);
            let mut left_vec = objects;
            let right_vec = left_vec.split_off(object_span / 2);
            Self::new(
                Some(Box::new(Self::new_from_vec(left_vec, tm, dur))),
                Some(Box::new(Self::new_from_vec(right_vec, tm, dur))),
                tm,
                dur,
            )
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb_box.hit(ray, t_min, t_max) {
            return None;
        }

        let mut hit_rec = None;
        let mut closest_so_far = t_max;

        if let Some(hit_left) = self.left.as_ref().unwrap().hit(ray, t_min, closest_so_far) {
            closest_so_far = hit_left.t;
            hit_rec = Some(hit_left);
        }
        if self.right.is_some() {
            if let Some(hit_right) = self.right.as_ref().unwrap().hit(ray, t_min, closest_so_far) {
                hit_rec = Some(hit_right);
            }
        }

        hit_rec
    }

    fn bounding_box(&self, _time: f64, _dur: f64) -> Option<AABB> {
        Some(self.aabb_box)
    }
}
