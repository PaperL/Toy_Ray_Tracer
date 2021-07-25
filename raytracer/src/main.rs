use std::f64::INFINITY;
use std::rc::Rc;

use hittable::moving_sphere::MovingSphere;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::{prelude::ThreadRng, random, Rng};

pub mod basic;
pub mod bvh;
pub mod hittable;
pub mod material;
pub mod texture;

use basic::INFINITESIMAL;
use basic::{
    camera::Camera,
    ray::Ray,
    vec3::{Point3, RGBColor, Vec3},
};

use hittable::{sphere::Sphere, Hittable, HittableList};

use material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

use texture::{
    checker_texture::CheckerTexture, image_texture::ImageTexture, solid_color::SolidColor,
};

//---------------------------------------------------------------------------------

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> RGBColor {
    if depth <= 0 {
        return RGBColor::default();
    }
    let tmp = ray.tm;

    if let Some(rec) = world.hit(ray, INFINITESIMAL, INFINITY) {
        let mut attenuation = RGBColor::default();
        if let Some(scattered) = rec.mat_ptr.scatter(ray, &rec, &mut attenuation) {
            if (scattered.tm - tmp).abs() > INFINITESIMAL {
                print!("ERRRR!!!!!!!!");
            }
            return ray_color(&scattered, world, depth - 1) * attenuation;
        } else {
            return RGBColor::default();
        }
        // let target: Point3 = rec.p + Vec3::rand_in_unit_hemisphere(&rec.normal);
        // return ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5;
    }

    let unit_dir = ray.dir.unit_vector();
    let t = 0.5 * (unit_dir.y + 1.);
    RGBColor::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
}

//---------------------------------------------------------------------------------

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let checker = Rc::new(CheckerTexture {
        odd: Rc::new(SolidColor::new(0.2, 0.3, 0.1)),
        even: Rc::new(SolidColor::new(0.9, 0.9, 0.9)),
    });

    let material_ground = Rc::new(Lambertian { albedo: checker });
    world.add(Sphere {
        cen: Point3::new(0., -1000., 0.),
        r: 1000.,
        mat_ptr: material_ground,
    });

    let mut rnd: ThreadRng = rand::thread_rng();

    for i in -7..7 {
        for j in -7..7 {
            let cen = Point3::new(
                i as f64 + 1.1 * rnd.gen::<f64>(),
                rnd.gen_range(0.17..0.23),
                j as f64 + 1.1 * rnd.gen::<f64>(),
            );

            if (cen - Point3::new(4., cen.y, 0.)).length() > 1.2 {
                let mat_dice = random::<f64>();
                if mat_dice < 0.8 {
                    let sphere_material = Rc::new(Lambertian {
                        albedo: Rc::new(SolidColor {
                            color_value: RGBColor::rand_1(),
                        }),
                    });
                    world.add(MovingSphere {
                        r: cen.y,
                        mat_ptr: sphere_material,
                        cen0: cen,
                        mov: Vec3::new(0., rnd.gen_range(0.0..0.5), 0.),
                        tm0: 0.,
                        dur: 1.,
                    });
                } else if mat_dice < 0.95 {
                    let sphere_material = Rc::new(Metal {
                        albedo: RGBColor::rand(0.5, 1.),
                        fuzz: rnd.gen_range(0.0..0.5),
                    });
                    world.add(Sphere {
                        cen,
                        r: cen.y,
                        mat_ptr: sphere_material,
                    });
                } else {
                    let sphere_material = Rc::new(Dielectric { ir: 1.5 });
                    world.add(Sphere {
                        cen,
                        r: cen.y,
                        mat_ptr: sphere_material,
                    });
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric { ir: 1.5 });
    world.add(Sphere {
        cen: Point3::new(-2., 1., 0.),
        r: 1.,
        mat_ptr: material1,
    });

    // let material2 = Rc::new(Lambertian {
    //     albedo: RGBColor::new(1., 0.95, 0.2),
    // });
    // world.add(Sphere {
    //     cen: Point3::new(-4., 1., 0.),
    //     r: 1.,
    //     mat_ptr: material2,
    // });

    let material3 = Rc::new(Metal {
        // albedo: RGBColor::new(0.97, 0.95, 0.9),
        // fuzz: 0.05,
        albedo: RGBColor::new(1., 1., 1.),
        fuzz: 0.,
    });
    world.add(Sphere {
        cen: Point3::new(4., 1., 0.),
        r: 1.,
        mat_ptr: material3,
    });

    world
}

fn two_spheres() -> HittableList {
    let mut objects = HittableList::default();

    let earth_texture = Rc::new(ImageTexture::new(&"texture/earth.jpg".to_string()));

    objects.add(Sphere {
        cen: Point3::new(0., -11., 0.),
        r: 10.,
        mat_ptr: Rc::new(Metal {
            albedo: RGBColor::new(0.5, 0.5, 0.6),
            fuzz: 0.05,
        }),
    });
    objects.add(Sphere {
        cen: Point3::new(0., 0.5, 0.),
        r: 1.5,
        mat_ptr: Rc::new(Lambertian {
            albedo: earth_texture,
        }),
    });

    objects
}

//---------------------------------------------------------------------------------
fn main() {
    print!("Initlizing...\t\t");

    //========================================================
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    let samples_per_pixel = 100;
    let max_depth = 50;

    //========================================================
    // World
    let world: HittableList = two_spheres();

    //========================================================
    // Camera
    let look_from = Point3::new(13., 2., 3.);
    let look_at = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let vfov = 20.;
    let aperture = 0.1;
    let focus_dist = 10.;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        0.,
        1.,
    );

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
