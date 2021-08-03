use std::f64::{INFINITY, NEG_INFINITY};

use rand::{prelude::ThreadRng, Rng};

use super::super::{HitRecord, Hittable};

use crate::{
    basic::{
        ray::Ray,
        vec3::{Point3, Vec3},
        INFINITESIMAL,
    },
    bvh::aabb::AABB,
    material::Material,
};

#[derive(Clone)]
pub struct Triangle<TM>
where
    TM: Material,
{
    pub ver: [Point3; 3], // 3 vertices of triangle
    pub normal: Vec3,
    pub mat: TM,
    pub cen: Point3, // 三角形中心
    pub area: f64,
    v: Vec3,
    w: Vec3,
    // v, w 用于计算线面相交, 具体原理见
    // https://blog.csdn.net/gyb641393267/article/details/48860189
    ab: Vec3,
    ac: Vec3,
}

impl<TM: Material> Triangle<TM> {
    pub fn new(ver: [Point3; 3], mat: TM) -> Self {
        let normal = (Vec3::cross(&ver[0], &ver[1])
            + Vec3::cross(&ver[1], &ver[2])
            + Vec3::cross(&ver[2], &ver[0]))
        .to_unit();
        let cen = (ver[0] + ver[1] + ver[2]) / 3.;

        let l0 = ((ver[0][0] - ver[1][0]).powi(2)
            + (ver[0][1] - ver[1][1]).powi(2)
            + (ver[0][2] - ver[1][2]).powi(2))
        .sqrt();
        let l1 = ((ver[1][0] - ver[2][1]).powi(2)
            + (ver[1][1] - ver[2][1]).powi(2)
            + (ver[1][2] - ver[2][2]).powi(2))
        .sqrt();
        let l2 = ((ver[2][0] - ver[0][0]).powi(2)
            + (ver[2][1] - ver[0][1]).powi(2)
            + (ver[2][2] - ver[0][2]).powi(2))
        .sqrt();
        let p = (l0 + l1 + l2) / 2.;
        let area = (p * (p - l0) * (p - l1) * (p - l2)).sqrt();

        let mut v = Vec3::cross(&normal, &(ver[1] - ver[0]));
        v /= Vec3::dot(&(ver[2] - ver[0]), &v);
        let mut w = Vec3::cross(&normal, &(ver[2] - ver[0]));
        w /= Vec3::dot(&(ver[1] - ver[0]), &w);

        Triangle {
            ver,
            normal,
            mat,
            area,
            cen,
            v,
            w,
            ab: ver[1] - ver[0],
            ac: ver[2] - ver[0],
        }
    }
}

impl<TM: Material> Hittable for Triangle<TM> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let orig = ray.orig;
        let dir = ray.dir;
        let n = self.normal;
        let cen = self.cen;
        // [(cx,cy,cz) - ((ox,oy,oz) + t * (dx,dy,dz))] · (nx,ny,nz) = 0
        let t = ((cen.x - orig.x) * n.x + (cen.y - orig.y) * n.y + (cen.z - orig.z) * n.z)
            / (dir.x * n.x + dir.y * n.y + dir.z * n.z);
        if t.is_nan() || t < t_min || t > t_max {
            // 直线与三角形平行
            return None;
        }

        let ap = (orig + dir * t) - self.ver[0];
        let gamma = Vec3::dot(&ap, &self.v);
        if gamma.is_sign_positive() && gamma < 1. {
            let beta = Vec3::dot(&ap, &self.w);
            if beta.is_sign_positive() && beta < 1. {
                let alpha = 1. - gamma - beta;
                if alpha.is_sign_positive() && alpha < 1. {
                    return Some(HitRecord::new(alpha, beta, t, ray, &n, &self.mat));
                }
            }
        }
        None
    }

    fn bounding_box(&self, _tm: f64, _dur: f64) -> Option<AABB> {
        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for v in self.ver {
            for i in 0..3 {
                if v[i] < min[i] {
                    min[i] = v[i];
                }
                if v[i] > max[i] {
                    max[i] = v[i]
                }
            }
        }

        Some(AABB::new(min, max))
    }

    fn pdf_value(&self, orig: &Point3, dir: &Vec3) -> f64 {
        if let Some(rec) = self.hit(&Ray::new(*orig, *dir, 0.), INFINITESIMAL, INFINITY) {
            let dis_sqrd = rec.t.powi(2) * dir.length_squared();
            let cosine = (Vec3::dot(dir, &rec.normal) / dir.length()).abs();

            Self::map_to(dis_sqrd / (cosine * self.area), 50., 5.)
        } else {
            Self::map_to(1000., 50., 5.)
        }
    }

    fn rand_dir(&self, orig: &Vec3) -> Vec3 {
        let mut rnd: ThreadRng = rand::thread_rng();
        let mut k1 = rnd.gen::<f64>();
        let mut k2 = rnd.gen::<f64>();
        if k1 + k2 > 1. {
            k1 = 1. - k1;
            k2 = 1. - k2;
        }

        (self.ab * k1 + self.ac * k2) - *orig
    }
}
