pub mod basic;
pub mod bvh;
pub mod hittable;
pub mod material;
pub mod pdf;
mod scene;
pub mod texture;

use std::{
    f64::INFINITY,
    sync::{mpsc, Arc},
    thread,
    time::Instant,
};

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use pdf::{cos_pdf::CosinePDF, hittable_pdf::HittablePDF, MixedPDF, PDF};
use rand::Rng;

use crate::{
    basic::{
        camera::Camera,
        ray::Ray,
        tp,
        vec3::{Point3, RGBColor, Vec3},
        INFINITESIMAL,
    },
    hittable::{Hittable, HittableList},
};

//---------------------------------------------------------------------------------

fn ray_color(
    ray: &Ray,
    world: Arc<dyn Hittable>,
    light: Arc<dyn Hittable>,
    background: &RGBColor,
    depth: i32,
) -> RGBColor {
    if depth <= 0 {
        return RGBColor::default();
    }
    if let Some(rec) = world.hit(ray, INFINITESIMAL, INFINITY) {
        let emitted = rec.mat.emitted(ray, &rec, rec.u, rec.v, rec.p);

        if let Some((albedo, _scattered, _pdf)) = rec.mat.scatter(ray, &rec) {
            let light_pdf = HittablePDF::new(rec.p, light.clone());
            let cos_pdf = CosinePDF::new(rec.normal);
            let mixed_pdf = MixedPDF::new(tp(cos_pdf), tp(light_pdf));

            let scattered = Ray::new(rec.p, mixed_pdf.generate(), ray.tm);
            let pdf_val = mixed_pdf.value(&scattered.dir);

            emitted
                + albedo
                    * rec.mat.scattering_pdf(&ray, &rec, &scattered)
                    * ray_color(&scattered, world, light, background, depth - 1)
                    / pdf_val
        } else {
            emitted
        }
    } else {
        *background
    }
}

//---------------------------------------------------------------------------------

fn main() {
    print!("{}[2J", 27 as char); // clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // set cursor at 1,1
    println!(
        "\n         {}  {}\n",
        style("PaperL's Toy Ray Tracer").cyan(),
        style("v0.4.4").yellow(),
    );
    println!(
        "{} 💿 {}",
        style("[1/5]").bold().dim(),
        style("Initlizing...").green()
    );
    let begin_time = Instant::now();

    const THREAD_NUMBER: usize = 5;

    // Image
    const ASPECT_RATIO: f64 = 1.;
    const IMAGE_WIDTH: usize = 2000;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    const SAMPLES_PER_PIXEL: u32 = 1000;
    const MAX_DEPTH: i32 = 50;
    const IMAGE_FORMAT: &str = "jpg";
    println!(
        "         Image size:              {}",
        style(IMAGE_WIDTH.to_string() + &"x".to_string() + &IMAGE_HEIGHT.to_string()).yellow()
    );
    println!(
        "         Sample number per pixel: {}",
        style(SAMPLES_PER_PIXEL.to_string()).yellow()
    );
    println!(
        "         Reflection max depth:    {}",
        style(MAX_DEPTH.to_string()).yellow()
    );

    // World
    let mut world = HittableList::default();
    let light;
    let mut light_list = HittableList::default();
    let mut background = RGBColor::default();

    // Camera
    let mut look_from = Point3::default();
    let mut look_at = Point3::default();
    let vup = Vec3::new(0., 1., 0.);
    let mut vfov = 0.;
    let aperture = 0.;
    let focus_dist = 1.;

    // Scene
    const SCENE_ID: i32 = 0;
    match SCENE_ID {
        0 => {
            scene::cornell_box_bvh(
                &mut world,
                &mut light_list,
                &mut background,
                &mut look_from,
                &mut look_at,
                &mut vfov,
            );
        }
        _ => {
            panic!("Unexpected SCENE_ID in main()!");
        }
    }

    if light_list.objects.len() > 1 {
        panic!("Have more than 1 objects in light_list!");
    } else {
        light = light_list.objects[0].clone();
    }

    // Camera
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

    //========================================================

    println!(
        "{} 🚀 {} {} {}",
        style("[2/5]").bold().dim(),
        style("Rendering with").green(),
        style(THREAD_NUMBER.to_string()).yellow(),
        style("Threads...").green(),
    );

    const SECTION_LINE_NUM: usize = IMAGE_HEIGHT / THREAD_NUMBER;

    let mut output_pixel_color = Vec::<RGBColor>::new();
    let mut thread_pool = Vec::<_>::new();

    let multiprogress = Arc::new(MultiProgress::new());
    multiprogress.set_move_cursor(true); // turn on this to reduce flickering

    for thread_id in 0..THREAD_NUMBER {
        let line_beg = SECTION_LINE_NUM * thread_id;
        let mut line_end = line_beg + SECTION_LINE_NUM;
        if line_end > IMAGE_HEIGHT || (thread_id == THREAD_NUMBER - 1 && line_end < IMAGE_HEIGHT) {
            line_end = IMAGE_HEIGHT;
        }

        let section_world = world.clone();
        let section_light = light.clone();

        let mp = multiprogress.clone();
        let progress_bar = mp.add(ProgressBar::new(
            ((line_end - line_beg) * IMAGE_WIDTH) as u64,
        ));
        progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {finished_lines}/{total_lines} ({eta})")
        .progress_chars("#>-"));

        let (tx, rx) = mpsc::channel();

        thread_pool.push((
            thread::spawn(move || {
                let mut progress = 0;
                progress_bar.set_position(0);

                let channel_send = tx;
                let world_ptr = tp(section_world);

                let mut section_pixel_color = Vec::<RGBColor>::new();

                let mut rnd = rand::thread_rng();

                for y in line_beg..line_end {
                    for x in 0..IMAGE_WIDTH {
                        let mut pixel_color = RGBColor::default();
                        for _i in 0..SAMPLES_PER_PIXEL {
                            let u = (x as f64 + rnd.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                            let v = (y as f64 + rnd.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                            let ray = cam.get_ray(u, v);
                            pixel_color += ray_color(
                                &ray,
                                world_ptr.clone(),
                                section_light.clone(),
                                &background,
                                MAX_DEPTH,
                            );
                        }
                        section_pixel_color.push(pixel_color);

                        progress += 1;
                        progress_bar.set_position(progress);
                    }
                }
                channel_send.send(section_pixel_color).unwrap();
                progress_bar.finish_with_message("Finished.");
            }),
            rx,
        ));
    }
    // 等待所有线程结束
    multiprogress.join().unwrap();

    //========================================================

    println!(
        "{} 🚛 {}",
        style("[3/5]").bold().dim(),
        style("Collecting Threads Results...").green(),
    );

    let collecting_progress_bar = ProgressBar::new(THREAD_NUMBER as u64);
    // join 和 recv 均会阻塞主线程
    for _thread_id in 0..THREAD_NUMBER {
        let thread = thread_pool.remove(0);
        match thread.0.join() {
            Ok(_) => {
                let mut received = thread.1.recv().unwrap();
                output_pixel_color.append(&mut received);
                collecting_progress_bar.inc(1);
            }
            Err(_) => {
                println!("{}", style("Joining the {}th thread failed!").bold().red());
            }
        }
    }
    collecting_progress_bar.finish_and_clear();

    //========================================================

    println!(
        "{} 🏭 {}",
        style("[4/5]").bold().dim(),
        style("Generating Image...").green()
    );

    let mut pixel_id = 0;
    for y in 0..IMAGE_HEIGHT as u32 {
        for x in 0..IMAGE_WIDTH as u32 {
            let pixel = img.get_pixel_mut(x, IMAGE_HEIGHT as u32 - y - 1);
            *pixel = image::Rgb(
                output_pixel_color[pixel_id]
                    .calc_color(SAMPLES_PER_PIXEL)
                    .to_u8_array(),
            );
            pixel_id += 1;
        }
    }

    //========================================================

    println!(
        "{} 🥽 {}",
        style("[5/5]").bold().dim(),
        style("Outping Image...").green()
    );

    img.save("output/output.".to_owned() + IMAGE_FORMAT)
        .unwrap();

    //========================================================

    println!(
        "\n      🎉 {}\n      🕒 Elapsed Time: {}",
        style("All Work Done.").bold().green(),
        style(HumanDuration(begin_time.elapsed())).yellow(),
    );
    println!("\n");
}
