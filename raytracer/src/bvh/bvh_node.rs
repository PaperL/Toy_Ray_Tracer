use std::{cmp::Ordering, sync::Arc};

use rand::Rng;

use super::aabb::AABB;
use crate::{
    basic::tp,
    hittable::{Hittable, HittableList},
};

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    aabb_box: AABB,
}

impl BvhNode {
    fn new(left: Arc<dyn Hittable>, right: Arc<dyn Hittable>, time: f64, dur: f64) -> Self {
        if let (Some(box_left), Some(box_right)) =
            (left.bounding_box(time, dur), right.bounding_box(time, dur))
        {
            Self {
                left,
                right,
                aabb_box: AABB::surrounding_box(&box_left, &box_right),
            }
        } else {
            panic!("No bounding box in BvhNode constructor.\n");
        }
    }

    pub fn new_from_list(hittable_list: &HittableList, tm: f64, dur: f64) -> Self {
        Self::new_from_vec(hittable_list.objects.clone(), tm, dur)
    }

    pub fn new_from_vec(mut objects: Vec<Arc<dyn Hittable>>, tm: f64, dur: f64) -> Self {
        let mut rnd = rand::thread_rng();
        let axis = rnd.gen_range(0..3);
        let comparator = |x: &Arc<dyn Hittable>, y: &Arc<dyn Hittable>| {
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
            Self::new(objects[0].clone(), objects[0].clone(), tm, dur)
        } else if object_span == 2 {
            match comparator(&objects[0], &objects[1]) {
                Ordering::Less => Self::new(objects[0].clone(), objects[1].clone(), tm, dur),
                _ => Self::new(objects[1].clone(), objects[0].clone(), tm, dur),
            }
        } else {
            objects.sort_unstable_by(comparator);
            let mut left_vec = objects;
            let right_vec = left_vec.split_off(object_span / 2);
            Self::new(
                tp(Self::new_from_vec(left_vec, tm, dur)),
                tp(Self::new_from_vec(right_vec, tm, dur)),
                tm,
                dur,
            )
        }
    }
}

impl Hittable for BvhNode {
    fn hit(
        &self,
        ray: &crate::basic::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        if !self.aabb_box.hit(ray, t_min, t_max) {
            return None;
        }

        let mut rec = None;
        let mut closest_so_far = t_max;

        if let Some(hit_left) = self.left.hit(ray, t_min, closest_so_far) {
            closest_so_far = hit_left.t;
            rec = Some(hit_left);
        }
        if let Some(hit_right) = self.right.hit(ray, t_min, closest_so_far) {
            rec = Some(hit_right);
        }

        rec
    }

    fn bounding_box(&self, _time: f64, _dur: f64) -> Option<AABB> {
        Some(self.aabb_box)
    }
}
