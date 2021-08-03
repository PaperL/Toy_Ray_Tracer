use rand::prelude::ThreadRng;
use rand::Rng;

use super::degree_to_radian;
use super::ray::Ray;
use super::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Camera {
    orig: Point3,
    corner: Point3, // lower left corner    画面左下角坐标
    hor: Vec3,      // horizontal           画面左下角到画面左上角向量
    ver: Vec3,      // vertical             画面左下角到画面右下角向量
    u: Vec3,        // u, v, w              用于计算透镜成像相关
    v: Vec3,        //
    lens_r: f64,    // lens radius          棱镜半径
    tm: f64,        // shutter open moment  快门开启时刻
    dur: f64,       // shutter open time    快门开启时间
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fr: Point3,        // look from
        to: Point3,        // look to
        vup: Vec3,         // view up vector, 用于确定画面上下方向与画面法相所在平面垂直于地面
        vfov: f64,         // vertical filed-of-view in degrees, 视域大小
        aspect_ratio: f64, // 图像长宽比
        aperture: f64,     // 光圈大小
        focus_dist: f64,   // 透镜到完美对焦平面的距离
        tm: f64,
        dur: f64,
    ) -> Self {
        let theta = degree_to_radian(vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (fr - to).to_unit();
        let u = Vec3::cross(&vup, &w);
        let v = Vec3::cross(&w, &u);

        let orig = fr;
        let hor = u * viewport_width * focus_dist;
        let ver = v * viewport_height * focus_dist;
        let corner = orig - hor / 2. - ver / 2. - w * focus_dist;

        Self {
            orig,
            corner,
            hor,
            ver,
            u,
            v,
            lens_r: aperture / 2.,
            tm,
            dur,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::rand_unit_disk() * self.lens_r;
        let offset = self.u * rd.x + self.v * rd.y;

        let mut rnd: ThreadRng = rand::thread_rng();
        Ray::new(
            self.orig + offset,
            self.corner + self.hor * s + self.ver * t - self.orig - offset,
            rnd.gen_range(self.tm..(self.tm + self.dur)),
        )
    }
}
