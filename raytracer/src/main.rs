pub mod basic;
pub mod bvh;
pub mod hittable;
pub mod material;
pub mod texture;

use std::{f64::INFINITY, rc::Rc};

use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rand::{prelude::ThreadRng, random, Rng};

use crate::{
    basic::{
        camera::Camera,
        ray::Ray,
        vec3::{Point3, RGBColor, Vec3},
        INFINITESIMAL,
    },
    bvh::bvh_node::BvhNode,
    hittable::{
        constant_medium::ConstantMedium, cube::Cube, moving_sphere::MovingSphere,
        rectangle::Rectangle, sphere::Sphere, Hittable, HittableList, RotateY, Translate,
    },
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    },
    texture::{
        checker_texture::CheckerTexture, image_texture::ImageTexture, solid_color::SolidColor,
    },
};

//---------------------------------------------------------------------------------

fn ray_color(ray: &Ray, world: Rc<dyn Hittable>, background: &RGBColor, depth: i32) -> RGBColor {
    if depth <= 0 {
        return RGBColor::default();
    }
    let tmp = ray.tm;

    if let Some(rec) = world.hit(ray, INFINITESIMAL, INFINITY) {
        let emitted = rec.mat_ptr.emitted(rec.u, rec.v, rec.p);
        if let Some((scattered, attenuation)) = rec.mat_ptr.scatter(ray, &rec) {
            if (scattered.tm - tmp).abs() > INFINITESIMAL {
                print!("ERRRR!!!!!!!!");
            }
            emitted + ray_color(&scattered, world, background, depth - 1) * attenuation
        } else {
            emitted
        }
    } else {
        *background
    }
}

//---------------------------------------------------------------------------------

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let checker = Rc::new(CheckerTexture {
        odd: Rc::new(SolidColor::new_from_value(0.2, 0.3, 0.1)),
        even: Rc::new(SolidColor::new_from_value(0.9, 0.9, 0.9)),
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

    let earth_texture = Rc::new(ImageTexture::new_from_file(
        &"texture/earth.jpg".to_string(),
    ));

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

fn simple_dark() -> HittableList {
    let mut objects = HittableList::default();

    let earth_texture = Rc::new(ImageTexture::new_from_file(
        &"texture/earth.jpg".to_string(),
    ));

    let solid_texture = Rc::new(SolidColor::new_from_value(1.0, 1.0, 0.9));

    objects.add(Sphere {
        cen: Point3::new(0., -1000., 0.),
        r: 1000.,
        mat_ptr: Rc::new(Lambertian {
            albedo: solid_texture,
        }),
    });
    objects.add(Sphere {
        cen: Point3::new(0., 2., 0.),
        r: 1.7,
        mat_ptr: Rc::new(Lambertian {
            albedo: earth_texture,
        }),
    });

    let light_texture = Rc::new(DiffuseLight::new_from_color(RGBColor::new(4., 4., 4.3)));

    objects.add(Rectangle::new(
        0,
        -1.5,
        1.5,
        1.,
        3.,
        -4.,
        light_texture.clone(),
    ));
    objects.add(Rectangle::new(
        1,
        1.,
        3.,
        -1.5,
        1.5,
        -4.,
        light_texture.clone(),
    ));
    objects.add(Rectangle::new(
        2,
        -1.5,
        1.5,
        -1.5,
        1.5,
        4.,
        light_texture.clone(),
    ));
    objects.add(Rectangle::new(2, -1.5, 1.5, -1.5, 1.5, 0.1, light_texture));

    objects
}

fn cornell_box() -> HittableList {
    let mut objects = HittableList::default();

    let red = Rc::new(Lambertian {
        albedo: Rc::new(SolidColor::new_from_value(0.65, 0.05, 0.05)),
    });
    let green = Rc::new(Lambertian {
        albedo: Rc::new(SolidColor::new_from_value(0.12, 0.45, 0.15)),
    });
    let white = Rc::new(Lambertian {
        albedo: Rc::new(SolidColor::new_from_value(0.73, 0.73, 0.73)),
    });
    let light = Rc::new(DiffuseLight::new_from_color(RGBColor::new(15., 15., 15.)));

    objects.add(Rectangle::new(1, 0., 555., 0., 555., 0., red));
    objects.add(Rectangle::new(1, 0., 555., 0., 555., 555., green));
    objects.add(Rectangle::new(2, 0., 555., 0., 555., 0., white.clone()));
    objects.add(Rectangle::new(2, 0., 555., 0., 555., 555., white.clone()));
    objects.add(Rectangle::new(0, 0., 555., 0., 555., 555., white.clone()));

    objects.add(Rectangle::new(2, 213., 343., 227., 332., 554., light));

    let cube1 = Cube::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        white.clone(),
    );

    let cube2 = Cube::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white,
    );

    let moved_cube1 = Translate {
        hit_ptr: Rc::new(RotateY::new(Rc::new(cube1), 15.)),
        offset: Vec3::new(265., 0., 295.),
    };

    let moved_cube2 = Translate {
        hit_ptr: Rc::new(RotateY::new(Rc::new(cube2), -18.)),
        offset: Vec3::new(130., 0., 65.),
    };

    objects.add(ConstantMedium::new_from_color(
        Rc::new(moved_cube1),
        0.01,
        RGBColor::new(0., 0., 0.),
    ));
    objects.add(ConstantMedium::new_from_color(
        Rc::new(moved_cube2),
        0.01,
        RGBColor::new(1., 1., 1.),
    ));

    objects
}

fn cornell_box_bvh() -> HittableList {
    let mut objects = HittableList::default();

    let red = Rc::new(Lambertian {
        albedo: Rc::new(SolidColor::new_from_value(0.65, 0.05, 0.05)),
    });
    let green = Rc::new(Lambertian {
        albedo: Rc::new(SolidColor::new_from_value(0.12, 0.45, 0.15)),
    });
    let white = Rc::new(Lambertian {
        albedo: Rc::new(SolidColor::new_from_value(0.73, 0.73, 0.73)),
    });
    let light = Rc::new(DiffuseLight::new_from_color(RGBColor::new(15., 15., 15.)));

    objects.add(Rectangle::new(1, 0., 555., 0., 555., 0., red));
    objects.add(Rectangle::new(1, 0., 555., 0., 555., 555., green));
    objects.add(Rectangle::new(2, 0., 555., 0., 555., 0., white.clone()));
    objects.add(Rectangle::new(2, 0., 555., 0., 555., 555., white.clone()));
    objects.add(Rectangle::new(0, 0., 555., 0., 555., 555., white.clone()));

    objects.add(Rectangle::new(2, 213., 343., 227., 332., 554., light));

    let cube1 = Cube::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 330., 165.),
        white.clone(),
    );

    let cube2 = Cube::new(
        Point3::new(0., 0., 0.),
        Point3::new(165., 165., 165.),
        white,
    );

    let moved_cube1 = Translate {
        hit_ptr: Rc::new(RotateY::new(Rc::new(cube1), 15.)),
        offset: Vec3::new(265., 0., 295.),
    };

    let moved_cube2 = Translate {
        hit_ptr: Rc::new(RotateY::new(Rc::new(cube2), -18.)),
        offset: Vec3::new(130., 0., 65.),
    };

    objects.add(ConstantMedium::new_from_color(
        Rc::new(moved_cube1),
        0.01,
        RGBColor::new(0., 0., 0.),
    ));
    objects.add(ConstantMedium::new_from_color(
        Rc::new(moved_cube2),
        0.01,
        RGBColor::new(1., 1., 1.),
    ));

    // objects
    let mut ret = HittableList::default();
    ret.add(BvhNode::new_from_list(&objects, 0., 1.));

    ret
}

fn book2_final() -> HittableList {
    let mut ground = HittableList::default();
    let mut objects = HittableList::default();

    // ground cubes
    let boxes_per_side = 20;
    let mut rnd: ThreadRng = rand::thread_rng();
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.;
            let x0 = -1000. + i as f64 * w;
            let z0 = -1000. + j as f64 * w;
            let y0 = 0.;
            let x1 = x0 + w;
            let y1 = rnd.gen_range(1.0..101.0);
            let z1 = z0 + w;
            let ground_material = Rc::new(Lambertian::new_from_color(
                RGBColor::new(0.48, 0.83, 0.53) * rnd.gen_range(0.9..1.1),
            ));
            ground.add(Cube::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground_material,
            ));
        }
    }
    objects.add(BvhNode::new_from_list(&ground, 0., 1.));

    // light
    let light = Rc::new(DiffuseLight::new_from_color(RGBColor::new(7., 7., 7.)));
    objects.add(Rectangle::new(2, 123., 423., 147., 412., 554., light));

    // moving yellow ball
    let cen0 = Point3::new(330., 400., 220.);
    let cen1 = cen0 + Vec3::new(30., 0., 0.);
    let moving_sphere_material = Rc::new(Lambertian::new_from_color(RGBColor::new(0.7, 0.3, 0.1)));
    objects.add(MovingSphere::new(
        cen0,
        cen1,
        0.,
        1.,
        50.,
        moving_sphere_material,
    ));

    // glass ball
    objects.add(Sphere::new(
        Point3::new(240., 170., 20.),
        60.,
        Rc::new(Dielectric::new(1.5)),
    ));

    // iron ball
    objects.add(Sphere::new(
        Point3::new(80., 150., 10.),
        50.,
        Rc::new(Metal::new(RGBColor::new(0.8, 0.8, 0.9), 1.0)),
    ));

    // smooth blue ball
    let boundary1 = Sphere::new(
        Point3::new(350., 120., 155.),
        50.,
        Rc::new(Dielectric::new(1.5)),
    );
    objects.add(boundary1.clone());
    objects.add(ConstantMedium::new(
        Rc::new(boundary1),
        0.2,
        Rc::new(SolidColor::new_from_value(0.2, 0.4, 0.9)),
    ));

    // air
    let boundary2 = Sphere::new(
        Point3::new(0., 0., 0.),
        5000.,
        Rc::new(Lambertian::new_from_color(RGBColor::default())),
    );
    objects.add(ConstantMedium::new(
        Rc::new(boundary2),
        0.0001,
        Rc::new(SolidColor::new_from_value(1., 1., 1.)),
    ));

    // globe
    let earth_texture = Rc::new(ImageTexture::new_from_file(
        &"texture/earth.jpg".to_string(),
    ));
    objects.add(Sphere::new(
        Point3::new(380., 220., 400.),
        120.,
        Rc::new(Lambertian::new(earth_texture)),
    ));

    // plastic foam
    let mut plastic_foam_list = HittableList::default();
    let white = Rc::new(Lambertian::new_from_color(RGBColor::new(0.73, 0.73, 0.73)));
    for _i in 0..1000 {
        plastic_foam_list.add(Sphere::new(Vec3::rand_1() * 165., 10., white.clone()));
    }
    objects.add(Translate::new(
        Rc::new(RotateY::new(
            Rc::new(BvhNode::new_from_list(&plastic_foam_list, 0., 1.)),
            15.,
        )),
        Vec3::new(-100., 270., 395.),
    ));

    objects
}

//---------------------------------------------------------------------------------
fn main() {
    print!("Initlizing...\t\t");

    //========================================================
    // Image
    const ASPECT_RATIO: f64 = 1.;
    const IMAGE_WIDTH: u32 = 2000;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    const SAMPLES_PER_PIXEL: u32 = 2000;
    const MAX_DEPTH: i32 = 70;

    //========================================================
    // World
    let world = Rc::new(book2_final());
    let background = RGBColor::new(0., 0., 0.);

    //========================================================
    // Camera
    let look_from = Point3::new(478., 278., -600.);
    let look_at = Point3::new(278., 278., 0.);
    // let look_from = Point3::new(278., 278., -800.);
    // let look_at = Point3::new(278., 278., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let vfov = 40.;
    let aperture = 0.;
    let focus_dist = 1.;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        focus_dist,
        0.,
        1.,
    );

    println!("Done.");

    //========================================================
    // Render

    println!("Rendering Progress(Number of Line):");
    let bar = ProgressBar::new(IMAGE_HEIGHT as u64);
    // bar.set_style(ProgressStyle::default_spinner());
    bar.tick();

    let mut rnd = rand::thread_rng();
    // let mut pixels: [[RGBColor; image_width as usize]; image_height as usize];
    for y in 0..IMAGE_HEIGHT {
        for x in 0..IMAGE_WIDTH {
            let mut pixel_color = RGBColor::default();
            for _i in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + rnd.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (y as f64 + rnd.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(&ray, world.clone(), &background, MAX_DEPTH);
            }
            let pixel = img.get_pixel_mut(x, IMAGE_HEIGHT - y - 1);
            *pixel = image::Rgb(pixel_color.calc_color(SAMPLES_PER_PIXEL).to_u8_array());
        }
        bar.inc(1);
    }

    bar.finish();
    println!("Generating Image...\tDone.");
    print!("Outputing File...\t");
    img.save("output/output.jpg").unwrap();
    println!("Done.");
    //========================================================
}
