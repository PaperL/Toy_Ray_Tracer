pub mod camera;
pub mod ray;
pub mod vec3;

pub const PI: f64 = 3.1415926535897932385;
pub const INFINITESIMAL: f64 = 0.0000001;

// pub fn rand_1()->f64{
//     static mut RND: ThreadRng = rand::thread_rng();
//     RND.gen()
// }

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
