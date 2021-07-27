pub mod basic;
pub mod bvh;
pub mod hittable;
pub mod material;
mod scene;
pub mod texture;

use std::{
    f64::INFINITY,
    sync::{mpsc, Arc},
    thread,
    time::Instant,
};

use console::{style, Emoji};
use image::{ImageBuffer, RgbImage};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
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

fn ray_color(ray: &Ray, item: Arc<dyn Hittable>, background: &RGBColor, depth: i32) -> RGBColor {
    if depth <= 0 {
        return RGBColor::default();
    }
    if let Some(rec) = item.hit(ray, INFINITESIMAL, INFINITY) {
        let emitted = rec.mat.emitted(rec.u, rec.v, rec.p);
        if let Some((scattered, attenuation)) = rec.mat.scatter(ray, &rec) {
            emitted + ray_color(&scattered, item, background, depth - 1) * attenuation
        } else {
            emitted
        }
    } else {
        *background
    }
}

//---------------------------------------------------------------------------------

static INIT: Emoji<'_, '_> = Emoji("💿  ", "");
static RUN: Emoji<'_, '_> = Emoji("🚀  ", "");
static COLLECT: Emoji<'_, '_> = Emoji("🚛  ", "");
static GENERATE: Emoji<'_, '_> = Emoji("🏭  ", "");
static OUTPUT: Emoji<'_, '_> = Emoji("🥽 ", "");
static FINISH: Emoji<'_, '_> = Emoji("🎉 ", "");
static TIME: Emoji<'_, '_> = Emoji("⏱ ", "");

fn main() {
    println!("{} {}Initlizing...", style("[1/5]").bold().dim(), INIT);
    let begin_time = Instant::now();

    const THREAD_NUMBER: usize = 8;

    // Image
    const ASPECT_RATIO: f64 = 1.;
    const IMAGE_WIDTH: usize = 1000;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    const SAMPLES_PER_PIXEL: u32 = 1000;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::default();
    let mut background = RGBColor::default();
    // Camera
    let mut look_from = Point3::default();
    let mut look_at = Point3::default();
    let vup = Vec3::new(0., 1., 0.);
    let mut vfov = 0.;
    let aperture = 0.;
    let focus_dist = 1.;

    const SCENE_ID: i32 = 4;
    match SCENE_ID {
        0 => {
            scene::random_scene(
                &mut world,
                &mut background,
                &mut look_from,
                &mut look_at,
                &mut vfov,
            );
        }
        1 => {
            scene::simple_dark_scene(
                &mut world,
                &mut background,
                &mut look_from,
                &mut look_at,
                &mut vfov,
            );
        }
        2 => {
            scene::cornell_box(
                &mut world,
                &mut background,
                &mut look_from,
                &mut look_at,
                &mut vfov,
            );
        }
        3 => {
            scene::cornell_box_bvh(
                &mut world,
                &mut background,
                &mut look_from,
                &mut look_at,
                &mut vfov,
            );
        }
        4 => {
            scene::book2_final_scene(
                &mut world,
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
        "{} {}Rendering with {} Threads...",
        style("[2/5]").bold().dim(),
        RUN,
        THREAD_NUMBER,
    );

    const SECTION_LINE_NUM: usize = IMAGE_HEIGHT / THREAD_NUMBER;

    let mut output_pixel_color = Vec::<RGBColor>::new();
    let mut thread_pool = Vec::<_>::new();

    let multiprogress = Arc::new(MultiProgress::new());
    for thread_id in 0..THREAD_NUMBER {
        let line_beg = SECTION_LINE_NUM * thread_id;
        let mut line_end = line_beg + SECTION_LINE_NUM;
        if line_end > IMAGE_HEIGHT || (thread_id == THREAD_NUMBER - 1 && line_end < IMAGE_HEIGHT) {
            line_end = IMAGE_HEIGHT;
        }

        let section_world = world.clone();

        let mp = multiprogress.clone();
        let progress_bar = mp.add(ProgressBar::new((line_end - line_beg) as u64));
        progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {finished_lines}/{total_lines} ({eta})")
        .progress_chars("#>-"));

        let (tx, rx) = mpsc::channel();

        thread_pool.push((
            thread::spawn(move || {
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
                            pixel_color +=
                                ray_color(&ray, world_ptr.clone(), &background, MAX_DEPTH);
                        }
                        section_pixel_color.push(pixel_color);
                    }
                    progress_bar.set_position((y - line_beg) as u64);
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
        "{} {}Collecting Threads Results...",
        style("[3/5]").bold().dim(),
        COLLECT,
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
        "{} {}Generating Image...",
        style("[4/5]").bold().dim(),
        GENERATE,
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

    println!("{} {}Outping Image...", style("[5/5]").bold().dim(), OUTPUT,);

    img.save("output/output.jpg").unwrap();

    //========================================================

    println!(
        " {} {}\n {} Elapsed Time: {}",
        FINISH,
        style("All Work Done.").bold().green(),
        TIME,
        HumanDuration(begin_time.elapsed()),
    );
}
