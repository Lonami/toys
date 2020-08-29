use crate::rand_f64;
use std::f64::consts;
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn new_in_range(low: f64, high: f64) -> Self {
        let gen = || low + (high - low) * rand_f64();
        Self {
            x: gen(),
            y: gen(),
            z: gen(),
        }
    }

    pub fn new_random_in_sphere() -> Self {
        loop {
            let vec = Self::new_in_range(-1.0, 1.0);
            if vec.len_sq() < 1.0 {
                break vec;
            }
        }
    }

    pub fn new_random_unit() -> Self {
        let a = rand_f64() * 2.0 * consts::PI;
        let z = rand_f64() * 2.0 - 1.0;
        let r = (1.0 - z.powi(2)).sqrt();
        Self {
            x: r * a.cos(),
            y: r * a.sin(),
            z,
        }
    }

    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    pub fn len_sq(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn unit(&self) -> Self {
        *self / self.len()
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn reflect(&self, normal: Vec3) -> Self {
        *self - 2.0 * self.dot(normal) * normal
    }

    // Based on Snell's law, after solving for the perpendicular and parallel lines
    pub fn refract(&self, normal: Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = (-*self).dot(normal);
        let r_out_perp = etai_over_etat * (*self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.len_sq()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
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

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        let inv = 1.0 / other;
        Self {
            x: self.x * inv,
            y: self.y * inv,
            z: self.z * inv,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Vec3) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Sum for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Vec3>,
    {
        let mut result = Self::default();
        for item in iter {
            result = result + item;
        }
        result
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
