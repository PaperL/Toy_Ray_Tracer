pub mod camera;
pub mod ray;
pub mod vec3;

use std::f64::consts::PI;

use rand::{prelude::ThreadRng, Rng};

pub const INFINITESIMAL: f64 = 0.0000001;

pub fn rand_1() -> f64 {
    let mut rnd: ThreadRng = rand::thread_rng();
    rnd.gen()
}

pub fn f64_equal(x: f64, y: f64) -> bool {
    (x - y).abs() < INFINITESIMAL
}

pub fn max_f64(x: f64, y: f64) -> f64 {
    if x > y {
        x
    } else {
        y
    }
}

pub fn min_f64(x: f64, y: f64) -> f64 {
    if x < y {
        x
    } else {
        y
    }
}

pub fn degree_to_radian(degree: f64) -> f64 {
    degree * PI / 180.
}

// open interval clamp
pub fn clamp_oi(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    x
}

// half-open interval clamp (left-closed and right-open)
pub fn clamp_hoi(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    };
    if x > max {
        return max - INFINITESIMAL;
    };
    x
}

//====================================================

// #[cfg(test)]
// mod tests {
//     fn test() {}
// }
