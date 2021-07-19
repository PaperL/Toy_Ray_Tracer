use crate::{basic::degree_to_radian, ray::Ray, Point3, Vec3};

pub struct Camera {
    orig: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = degree_to_radian(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = Vec3::cross(&vup, &w);
        let v = Vec3::cross(&w, &u);

        let orig = look_from;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = orig - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            orig,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            orig: self.orig,
            dir: self.lower_left_corner + self.horizontal * s + self.vertical * t - self.orig,
        }
    }
}
