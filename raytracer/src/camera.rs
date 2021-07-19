use crate::{ray::Ray, Point3, Vec3};

pub struct Camera {
    orig: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_width = 4.0;
        let viewport_height = viewport_width / aspect_ratio;
        let focal_length = 1.0;

        let orig = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            orig - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            orig,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.orig,
            dir: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.orig,
        }
    }
}
