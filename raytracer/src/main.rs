use rand::Rng;
use std::f64::INFINITY;
use std::rc::Rc;

use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

mod basic;
use basic::INFINITESIMAL;

mod vec3;
use vec3::{Point3, RGBColor, Vec3};

mod ray;
use ray::Ray;

mod hittable;
use hittable::{Hittable, HittableList, Sphere};

mod camera;
use camera::Camera;

mod material;
use crate::material::{Dielectric, Lambertian, Metal};

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> RGBColor {
    if depth <= 0 {
        return RGBColor::default();
    }

    if let Some(rec) = world.hit(ray, INFINITESIMAL, INFINITY) {
        let mut attenuation = RGBColor::default();
        if let Some(scattered) = rec.mat_ptr.scatter(ray, &rec, &mut attenuation) {
            return ray_color(&scattered, world, depth - 1) * attenuation;
        } else {
            return RGBColor::default();
        }
        // let target: Point3 = rec.p + Vec3::rand_in_unit_hemisphere(&rec.normal);
        // return ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5;
    }

    let unit_dir = ray.dir.unit_vector();
    let t = 0.5 * (unit_dir.y + 1.0);
    RGBColor::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
//---------------------------------------------------------------------------------
fn main() {
    print!("Initlizing...\t\t");

    //========================================================
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 200;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let samples_per_pixel = 100;
    let max_depth = 50;

    //========================================================
    // World
    let mut world = HittableList::default();

    let material_ground = Rc::new(Lambertian {
        albedo: RGBColor {
            x: 0.8,
            y: 0.8,
            z: 0.0,
        },
    });

    let material_center = Rc::new(Lambertian {
        albedo: RGBColor {
            x: 0.1,
            y: 0.2,
            z: 0.5,
        },
    });

    let material_left = Rc::new(Dielectric { ir: 1.5 });

    let material_right = Rc::new(Metal {
        albedo: RGBColor {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
        fuzz: 1.0,
    });

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    //========================================================
    // Camera
    let cam = Camera::new();

    println!("Done.");

    //========================================================
    // Render

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
                pixel_color += ray_color(&ray, &world, max_depth);
            }

            let pixel = img.get_pixel_mut(x, image_height - y - 1);
            *pixel = image::Rgb(pixel_color.calc_color(samples_per_pixel).to_u8_array());
        }
        bar.inc(1);
    }

    bar.finish();
    println!("Generating Image...\tDone.");
    print!("Outputing File...\t");
    img.save("output/output.png").unwrap();
    println!("Done.");
    //========================================================
}
