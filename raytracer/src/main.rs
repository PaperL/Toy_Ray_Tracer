use rand::Rng;
use std::f64::INFINITY;

use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

mod basic;
pub use basic::{Point3, RGBColor, Vec3};

mod ray;
use ray::Ray;

mod hittable;
use hittable::{HitRecord, HittableList, Sphere};

mod camera;
use camera::Camera;

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
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::default();
    let s1 = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    world.add(&s1);
    world.add(&s2);

    // Camera
    let cam = Camera::new();

    println!("Done.");
    //========================================================
    println!("Rendering Progress(Number of Line):");
    let bar = ProgressBar::new(image_height as u64);
    // bar.set_style(ProgressStyle::default_spinner());
    // bar.tick();

    let mut rnd = rand::thread_rng();
    for y in 0..image_height {
        for x in 0..image_width {
            let mut pixel_color = RGBColor::default();
            for _i in 0..samples_per_pixel {
                let u = (x as f64 + rnd.gen::<f64>()) / (image_width - 1) as f64;
                let v = (y as f64 + rnd.gen::<f64>()) / (image_height - 1) as f64;
                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(ray, &world)
            }

            let pixel = img.get_pixel_mut(x, image_height - y - 1);
            *pixel = image::Rgb(pixel_color.calc_color(samples_per_pixel).to_u8_array());
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
