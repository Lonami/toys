use crate::Vec3;
use std::fmt;

pub struct Color(pub Vec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r = (255.999 * self.0.x) as i32;
        let g = (255.999 * self.0.y) as i32;
        let b = (255.999 * self.0.z) as i32;
        write!(f, "{} {} {}\n", r, g, b)
    }
}
