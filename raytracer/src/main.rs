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
        vec3::{Point3, RGBColor, Vec3},
        INFINITESIMAL,
    },
    hittable::{Hittable, HittableList},
    material::ScaRecData,
};

//---------------------------------------------------------------------------------

fn ray_color(
    ray: &Ray,
    world: &HittableList,
    lights: &HittableList,
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
            match sca_rec.dat {
                ScaRecData::Specular(ray) => {
                    sca_rec.attenutaion * ray_color(&ray, world, lights, background, depth - 1)
                }
                ScaRecData::Pdf(pdf) => {
                    let light_pdf = HittablePDF::new(hit_rec.p, lights);
                    let mixed_pdf = MixedPDF::new(pdf, light_pdf);

                    let pdf_dir = mixed_pdf.generate();
                    let pdf_val = mixed_pdf.value(&pdf_dir);
                    let scattered = Ray::new(hit_rec.p, pdf_dir, ray.tm);

                    let k = sca_rec.attenutaion
                        * hit_rec.mat.scattering_pdf(&ray, &hit_rec, &scattered)
                        / pdf_val;

                    if k.is_zero() {
                        emitted
                    } else {
                        emitted + k * ray_color(&scattered, world, lights, background, depth - 1)
                    }
                }
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
        "\n         {}    {}\n",
        style("PaperL's Toy Ray Tracer").cyan(),
        style(format!("v{}", env!("CARGO_PKG_VERSION"))).yellow(),
    );
    println!(
        "{} 💿 {}",
        style("[1/5]").bold().dim(),
        style("Initlizing...").green()
    );
    let begin_time = Instant::now();

    const THREAD_NUMBER: usize = 7;

    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: usize = 1920;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    const SAMPLES_PER_PIXEL: u32 = 1500;
    const MAX_DEPTH: i32 = 60;

    // const HALO_SIZE: i32 = IMAGE_WIDTH as i32 / 10;

    const JPEG_QUALITY: u8 = 100;

    println!(
        "         Image size:                {}",
        style(IMAGE_WIDTH.to_string() + &"x".to_string() + &IMAGE_HEIGHT.to_string()).yellow()
    );
    println!(
        "         Sample number per pixel:   {}",
        style(SAMPLES_PER_PIXEL.to_string()).yellow()
    );
    println!(
        "         Reflection max depth:      {}",
        style(MAX_DEPTH.to_string()).yellow()
    );

    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    // World
    let background = RGBColor::new(0., 0., 0.);

    // Camera
    let look_from = Point3::new(-850., 80., 0.);
    let look_at = Point3::new(-780., 530., 800.);
    let vup = Vec3::new(0., 1., 0.);
    let vfov = 120.;
    let aperture = 0.;
    let focus_dist = 1.;
    let distortion = -0.05;

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
        distortion,
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

        // Secene
        let mut section_world = HittableList::default();
        let mut section_lights = HittableList::default();
        scene::paper_world(&mut section_world, &mut section_lights);

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
                                &section_world,
                                &section_lights,
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
    /*
    let mut halo_cnt = 0;
    let mut light_pixel_cnt = 0;
    let mut halo = vec![vec![RGBColor::default(); IMAGE_WIDTH]; IMAGE_HEIGHT];
    for y in 0..IMAGE_HEIGHT as u32 {
        for x in 0..IMAGE_WIDTH as u32 {
            let pixel_color = output_pixel_color[pixel_id] / SAMPLES_PER_PIXEL as f64;
            let sum = pixel_color.x + pixel_color.y + pixel_color.z;
            if sum > 4. {
                light_pixel_cnt += 1;

                let mut y1 = y as i32 - HALO_SIZE;
                let mut y2 = y as i32 + HALO_SIZE;
                let mut x1 = x as i32 - HALO_SIZE;
                let mut x2 = x as i32 + HALO_SIZE;

                if y1 < 0 {
                    y1 = 0;
                }
                if y2 > IMAGE_HEIGHT as i32 {
                    y2 = IMAGE_HEIGHT as i32;
                }
                if x1 < 0 {
                    x1 = 0;
                }
                if x2 > IMAGE_WIDTH as i32 {
                    x2 = IMAGE_WIDTH as i32;
                }

                for ty in y1..y2 {
                    for tx in x1..x2 {
                        let h = ((tx - x as i32) * (ty - y as i32)).abs();
                        const MAX_MULTIPLE: i32 = HALO_SIZE * 4;
                        if h <= MAX_MULTIPLE {
                            if halo[ty as usize][tx as usize].is_zero() {
                                halo_cnt += 1;
                            }
                            halo[ty as usize][tx as usize] += pixel_color / sum
                                * f64::atan((MAX_MULTIPLE - h) as f64 * (4. / MAX_MULTIPLE as f64))
                                * 2.
                                / PI;
                        }
                    }
                }
            }
            pixel_id += 1;
        }
    }
    println!(
        "         Number of light pixels:    {:6} ({} of all pixels)",
        style(light_pixel_cnt.to_string()).yellow(),
        style(format!(
            "{:.2}%",
            (light_pixel_cnt as f64 / (IMAGE_WIDTH * IMAGE_HEIGHT) as f64)
        ))
        .yellow(),
    );
    println!(
        "         Number of pixels of halo:  {:6} ({} of all pixels)",
        style(halo_cnt.to_string()).yellow(),
        style(format!(
            "{:.2}%",
            (halo_cnt as f64 / (IMAGE_WIDTH * IMAGE_HEIGHT) as f64)
        ))
        .yellow(),
    );

    pixel_id = 0;*/
    for y in 0..IMAGE_HEIGHT as u32 {
        for x in 0..IMAGE_WIDTH as u32 {
            let pixel_color = output_pixel_color[pixel_id].calc_color(SAMPLES_PER_PIXEL);
            // + halo[y as usize][x as usize];

            let pixel = img.get_pixel_mut(x, IMAGE_HEIGHT as u32 - y - 1);
            *pixel = image::Rgb(pixel_color.to_u8_array());

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
        "         Image format:              {}",
        style("JPEG").yellow()
    );
    println!(
        "         JPEG image quality:        {}",
        style(JPEG_QUALITY.to_string()).yellow()
    );

    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create("raytracer/output/output.jpg").unwrap();
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
