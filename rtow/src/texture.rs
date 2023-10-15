use crate::{Color, Perlin, Vec3};
use jpeg_decoder::{Decoder, PixelFormat};
use std::fs::File;
use std::io;
use std::path::Path;

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

pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
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

impl ImageTexture {
    fn empty() -> Self {
        Self {
            data: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("failed to open file: {:?}: {}", path, e);
                return Self::empty();
            }
        };

        let reader = io::BufReader::new(file);
        let mut decoder = Decoder::new(reader);
        let pixels = match decoder.decode() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("failed to read image: {:?}: {}", path, e);
                return Self::empty();
            }
        };
        let metadata = decoder.info().unwrap();
        if metadata.pixel_format != PixelFormat::RGB24 {
            eprintln!(
                "only rgb with 8 bits per channel images are supported: {:?}",
                path
            );
            return Self::empty();
        }
        Self {
            data: pixels,
            width: metadata.width as usize,
            height: metadata.height as usize,
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
        Color(
            Vec3::new(1.0, 1.0, 1.0)
                // May obtain negative values, shift them to be positive
                * 0.5
                * (1.0 + f64::sin(self.scale * point.z + 10.0 * self.perlin.turbulence(point, 7))),
        )
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _point: Vec3) -> Color {
        if self.data.is_empty() {
            // Use solid cyan to help debugging
            return Color::new(0.0, 1.0, 1.0);
        }

        let clamp = |x: f64, min, max| x.min(max).max(min);

        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0); // flip v to image coordinates

        let mut i = (u * self.width as f64) as usize;
        let mut j = (v * self.height as f64) as usize;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        // Using RGB24, 3 bytes per pixel
        let idx = (j * 3 * self.width + i * 3) as usize;
        let rgb = &self.data[idx..idx + 3];
        let scale = 1.0 / 255.0;

        Color::new(
            scale * rgb[0] as f64,
            scale * rgb[1] as f64,
            scale * rgb[2] as f64,
        )
    }
}
