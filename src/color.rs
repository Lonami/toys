use crate::Vec3;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Color(pub Vec3);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }

    pub fn as_bytes(&self) -> [u8; 3] {
        let r = (255.999 * self.0.x) as u8;
        let g = (255.999 * self.0.y) as u8;
        let b = (255.999 * self.0.z) as u8;
        [r, g, b]
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [r, g, b] = self.as_bytes();
        write!(f, "{} {} {}\n", r, g, b)
    }
}
