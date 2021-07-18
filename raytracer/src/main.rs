use std::f64::INFINITY;

use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

mod basic;
pub use basic::{Point3, RGBColor, Vec3};
mod ray;
use ray::Ray;
mod hittable;
use hittable::{HitRecord, HittableList};

use crate::hittable::Sphere;

// fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> f64 {
//     let oc = ray.orig - center.clone();
//     let a = ray.dir.length_squared();
//     let half_b = Vec3::dot(oc, ray.dir);
//     let c = oc.length_squared() - radius * radius;
//     let discriminant = half_b * half_b - a * c;
//     if discriminant < 0.0 {
//         return -1.0;
//     } else {
//         return (-half_b - discriminant.sqrt()) / a;
//     }
// }

fn ray_color(ray: Ray, world: &HittableList) -> RGBColor {
    let mut rec: HitRecord = Default::default();
    if world.hit_any(&ray, 0.0, INFINITY, &mut rec) {
        return (rec.normal + RGBColor::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_dir = ray.dir.unit_vector();
    let t = 0.5 * (unit_dir.y + 1.0);
    RGBColor::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
    //========================================================
    print!("Initlizing...\t\t");

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    // World
    let mut world: HittableList = Default::default();
    let s1 = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    world.add(&s1);
    world.add(&s2);

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("Done.");
    //========================================================
    println!("Rendering Progress:");
    let bar = ProgressBar::new(image_height as u64);

    for y in 0..image_height {
        for x in 0..image_width {
            let u = x as f64 / (image_width - 1) as f64;
            let v = y as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + (horizontal * u) + (vertical * v) - origin,
            );
            let pixel_color = ray_color(r, &world);

            let pixel = img.get_pixel_mut(x, image_height - y - 1);
            *pixel = image::Rgb((pixel_color * 255.99999).to_u8_array());
        }
        bar.inc(1);
    }

    bar.finish();
    println!("Generating Image...\tDone.");
    //========================================================
    print!("Outputing File...\t");
    img.save("output/test.png").unwrap();
    println!("Done.");
    //========================================================
}
