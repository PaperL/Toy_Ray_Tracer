pub mod basic;
pub mod bvh;
pub mod hittable;
pub mod material;
pub mod pdf;
mod scene;
pub mod texture;

use std::{
    f64::INFINITY,
    fmt::Display,
    fs::File,
    process::exit,
    sync::{mpsc, Arc},
    thread,
    time::Instant,
};

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use pdf::{hittable_pdf::HittablePDF, MixedPDF, PDF};
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
    lights: Arc<dyn Hittable>,
    background: &RGBColor,
    depth: i32,
) -> RGBColor {
    if depth <= 0 {
        return RGBColor::default();
    }
    if let Some(hit_rec) = world.hit(ray, INFINITESIMAL, INFINITY) {
        let emitted = hit_rec
            .mat
            .emitted(ray, &hit_rec, hit_rec.u, hit_rec.v, hit_rec.p);

        if let Some(sca_rec) = hit_rec.mat.scatter(ray, &hit_rec) {
            if let Some(specular_ray) = sca_rec.specular {
                sca_rec.attenutaion * ray_color(&specular_ray, world, lights, background, depth - 1)
            } else {
                let light_pdf = HittablePDF::new(hit_rec.p, lights.clone());
                let mixed_pdf = MixedPDF::new(sca_rec.pdf.unwrap(), tp(light_pdf));

                let pdf_dir = mixed_pdf.generate();
                let pdf_val = mixed_pdf.value(&pdf_dir);
                let scattered = Ray::new(hit_rec.p, pdf_dir, ray.tm);

                emitted
                    + sca_rec.attenutaion
                        * hit_rec.mat.scattering_pdf(&ray, &hit_rec, &scattered)
                        * ray_color(&scattered, world, lights, background, depth - 1)
                        / pdf_val
            }
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
        style(format!("v{}", env!("CARGO_PKG_VERSION"))).yellow(),
    );
    println!(
        "{} 💿 {}",
        style("[1/5]").bold().dim(),
        style("Initlizing...").green()
    );
    let begin_time = Instant::now();

    const THREAD_NUMBER: usize = 4;

    // Image
    const ASPECT_RATIO: f64 = 1.;
    const IMAGE_WIDTH: usize = 300;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    const SAMPLES_PER_PIXEL: u32 = 1000;
    const MAX_DEPTH: i32 = 50;
    const JPEG_QUALITY: u8 = 80;
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
    let mut lights = HittableList::default();
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
                &mut lights,
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
        let section_lights = lights.clone();

        let mp = multiprogress.clone();
        let progress_bar = mp.add(ProgressBar::new(
            ((line_end - line_beg) * IMAGE_WIDTH) as u64,
        ));
        progress_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

        let (tx, rx) = mpsc::channel();

        thread_pool.push((
            thread::spawn(move || {
                let mut progress = 0;
                progress_bar.set_position(0);

                let channel_send = tx;
                let world_ptr = tp(section_world);
                let lights_ptr = tp(section_lights);

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
                                lights_ptr.clone(),
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

    let mut thread_finish_successfully = true;
    let collecting_progress_bar = ProgressBar::new(THREAD_NUMBER as u64);
    // join 和 recv 均会阻塞主线程
    for thread_id in 0..THREAD_NUMBER {
        let thread = thread_pool.remove(0);
        match thread.0.join() {
            Ok(_) => {
                let mut received = thread.1.recv().unwrap();
                output_pixel_color.append(&mut received);
                collecting_progress_bar.inc(1);
            }
            Err(_) => {
                thread_finish_successfully = false;
                println!(
                    "      ⚠️ {}{}{}",
                    style("Joining the ").red(),
                    style(thread_id.to_string()).yellow(),
                    style("th thread failed!").red(),
                );
            }
        }
    }
    if !thread_finish_successfully {
        exit_with_error("Get run-time error!");
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
    println!(
        "         Image format:            {}",
        style("JPEG").yellow()
    );
    println!(
        "         JPEG image quality:      {}",
        style(JPEG_QUALITY.to_string()).yellow()
    );

    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create("output/output.jpg").unwrap();
    match output_image.write_to(
        &mut output_file,
        image::ImageOutputFormat::Jpeg(JPEG_QUALITY),
    ) {
        Ok(_) => {}
        Err(_) => exit_with_error("Outputing file failed!"),
    }

    //========================================================

    println!(
        "\n      🎉 {}\n      🕒 Elapsed Time: {}",
        style("All Work Done.").bold().green(),
        style(HumanDuration(begin_time.elapsed())).yellow(),
    );
    println!("\n");

    exit(0);
}

fn exit_with_error<T>(info: T)
where
    T: Display,
{
    println!(
        "\n\n      {}{}\n\n",
        style("❌ Error: ").bold().red().on_yellow(),
        style(info).black().on_yellow()
    );
    exit(1);
}
