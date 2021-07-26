use std::{cmp::Ordering, rc::Rc};

use rand::{prelude::ThreadRng, Rng};

use super::aabb::AABB;
use crate::hittable::{Hittable, HittableList};

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    aabb_box: AABB,
}

impl BvhNode {
    fn new(left: Rc<dyn Hittable>, right: Rc<dyn Hittable>, time: f64, dur: f64) -> Self {
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

    pub fn new_from_list(hittable_list: &HittableList, time: f64, dur: f64) -> Self {
        Self::new_from_vec(hittable_list.objects.clone(), time, dur)
    }

    pub fn new_from_vec(mut objects: Vec<Rc<dyn Hittable>>, time: f64, dur: f64) -> Self {
        let mut rnd: ThreadRng = rand::thread_rng();
        let axis = rnd.gen_range(0..3);
        // let comparator = |x, y| BvhNode::box_compare(x, y, axis);
        let comparator = |x: &Rc<dyn Hittable>, y: &Rc<dyn Hittable>| {
            f64::partial_cmp(
                &(x.bounding_box(0., 0.).unwrap().min[axis]),
                &(y.bounding_box(0., 0.).unwrap().min[axis]),
            )
            .unwrap()
        };

        let object_span = objects.len();
        if object_span == 0 {
            panic!("Get empty Vec at BvhNode::new_from_vec!");
        } else if object_span == 1 {
            Self::new(objects[0].clone(), objects[0].clone(), time, dur)
        } else if object_span == 2 {
            match comparator(&objects[0], &objects[1]) {
                Ordering::Less => Self::new(objects[0].clone(), objects[1].clone(), time, dur),
                _ => Self::new(objects[1].clone(), objects[0].clone(), time, dur),
            }
        } else {
            objects.sort_unstable_by(comparator);
            let mut left_vec = objects;
            let right_vec = left_vec.split_off(object_span / 2);
            Self::new(
                Rc::new(Self::new_from_vec(left_vec, time, dur)),
                Rc::new(Self::new_from_vec(right_vec, time, dur)),
                time,
                dur,
            )
        }
    }

    // fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: usize) -> Ordering {
    //     if let (Some(box_a), Some(box_b)) = (a.bounding_box(0., 0.), b.bounding_box(0., 0.)) {
    //         return f64::partial_cmp(&box_a.min[axis], &box_b.min[axis]).unwrap();
    //     } else {
    //         panic!("No bounding box in BvhNode constructor.\n");
    //     }
    // }
}

impl Hittable for BvhNode {
    fn hit(
        &self,
        ray: &crate::basic::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        if !self.aabb_box.hit(ray, t_min, t_max) {
            None
        } else if let Some(hit_left) = self.left.hit(ray, t_min, t_max) {
            Some(hit_left)
        } else if let Some(hit_right) = self.right.hit(ray, t_min, t_max) {
            Some(hit_right)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time: f64, _dur: f64) -> Option<AABB> {
        Some(self.aabb_box)
    }
}
