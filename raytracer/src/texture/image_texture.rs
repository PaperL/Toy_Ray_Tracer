use image::{GenericImageView, RgbImage};

use crate::basic::{clamp_hoi, vec3::RGBColor};

use super::Texture;

pub struct ImageTexture {
    pub image: RgbImage,
    pub bytes_per_scanline: u32,
}

impl ImageTexture {
    pub fn new_from_file(file_name: &str) -> Self {
        let tmp_image;
        match image::open(file_name) {
            Ok(ret) => tmp_image = ret,
            Err(_) => panic!("Opening image fails! File name: \"{}\"", file_name),
        }

        ImageTexture {
            image: tmp_image.to_rgb8(),
            bytes_per_scanline: tmp_image.dimensions().0 * 3,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _p: crate::basic::vec3::Point3) -> RGBColor {
        u = clamp_hoi(u, 0., 1.);
        v = clamp_hoi(1. - v, 0., 1.);
        let i = (self.image.width() as f64 * u) as u32;
        let j = (self.image.height() as f64 * v) as u32;
        if i >= self.image.width() || j >= self.image.height() {
            panic!("Unexpected Error in ImageTexture::value");
        }

        let color_scale = 1.0 / 255.0;
        RGBColor::new(
            self.image.get_pixel(i, j).0[0] as f64 * color_scale,
            self.image.get_pixel(i, j).0[1] as f64 * color_scale,
            self.image.get_pixel(i, j).0[2] as f64 * color_scale,
        )
    }
}
