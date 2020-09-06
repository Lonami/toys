use crate::{Color, Perlin, Vec3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Color;
}

pub struct SolidColor {
    pub color: Color,
}

pub struct CheckerTexture {
    pub even: Box<dyn Texture>,
    pub odd: Box<dyn Texture>,
}

pub struct NoiseTexture {
    perlin: Perlin,
    scale: f64,
}

impl CheckerTexture {
    pub fn new(even: Color, odd: Color) -> Self {
        Self {
            even: Box::new(SolidColor { color: even }),
            odd: Box::new(SolidColor { color: odd }),
        }
    }
}

impl NoiseTexture {
    pub fn scaled(scale: f64) -> Self {
        Self {
            perlin: Perlin::new(),
            scale,
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: Vec3) -> Color {
        self.color
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Color {
        let sines = (10.0 * point.x).sin() * (10.0 * point.y).sin() * (10.0 * point.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, point: Vec3) -> Color {
        Color(Vec3::new(1.0, 1.0, 1.0) * self.perlin.noise(self.scale * point))
    }
}
