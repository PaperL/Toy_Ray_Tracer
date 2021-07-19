pub use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
pub use rand::{prelude::ThreadRng, random, Rng};

use crate::basic::{INFINITESIMAL,clamp_hoi};



pub type RGBColor = Vec3;
pub type Point3 = Vec3;

#[derive(Copy, Clone, PartialEq, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(op1: &Self, op2: &Self) -> f64 {
        op1.x * op2.x + op1.y * op2.y + op1.z * op2.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.x.abs() < INFINITESIMAL && self.y.abs() < INFINITESIMAL && self.z.abs() < INFINITESIMAL
    }

    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn rand_1() -> Self {
        let mut rnd: ThreadRng = rand::thread_rng();
        Self {
            // todo 减少 rand::thread_rng() 开销
            x: rnd.gen::<f64>(),
            y: rnd.gen::<f64>(),
            z: rnd.gen::<f64>(),
        }
    }

    pub fn rand(min: f64, max: f64) -> Self {
        let mut rnd: ThreadRng = rand::thread_rng();
        Self {
            x: rnd.gen_range(min..max),
            y: rnd.gen_range(min..max),
            z: rnd.gen_range(min..max),
        }
    }

    pub fn rand_unit() -> Self {
        Vec3::rand(-1.0, 1.0).unit_vector()
    }

    pub fn rand_in_unit_sphere() -> Self {
        Vec3::rand(-1.0, 1.0).unit_vector() * random::<f64>()
    }

    pub fn rand_in_unit_hemisphere(normal: &Vec3) -> Self {
        let p = Vec3::rand(-1.0, 1.0).unit_vector() * random::<f64>();
        return if Vec3::dot(&p, &*normal) > 0.0 { p } else { -p };
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        (*v) - (*n) * Vec3::dot(&v, &n) * 2.0
    }
}

impl RGBColor {
    pub fn to_u8_array(&self) -> [u8; 3] {
        [self.x as u8, self.y as u8, self.z as u8]
    }

    pub fn calc_color(&mut self, samples_per_pixel: u32) -> &Self {
        let scale = 1.0 / samples_per_pixel as f64;
        self.x = clamp_hoi((self.x * scale).sqrt() * 256.0, 0.0, 256.0);
        self.y = clamp_hoi((self.y * scale).sqrt() * 256.0, 0.0, 256.0);
        self.z = clamp_hoi((self.z * scale).sqrt() * 256.0, 0.0, 256.0);
        self
    }
}

impl Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
