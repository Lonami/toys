mod color;
mod ray;
mod vec3;

pub use color::Color;
pub use ray::Ray;
pub use vec3::Vec3;

use std::io::{self, BufWriter, Write};

// A point P is in the sphere at center C if:
//   (x - Cx)² + (y - Cy)² + (z - Cz)² = r²
//
// This equals the dot product of the vector from C to P:
//   (P - C) ⋅ (P - C) = r²
//
// We want to solve for our ray, which gives a point in P(t) = A + tB
//   (P(t) - C) ⋅ (P(t) - C) = r²
//   (A + tB - C) ⋅ (A + tB - C) = r²
//   t²B ⋅ B + 2tB ⋅ (A - C) + (A - C) ⋅ (A - C) - r² = 0
//   -------   ------------   ----------------------
//   2nd deg   1st degree     constant
//
// We have a quadratic equation in terms of the unknown t, which becomes:
//   at² + bx + c = 0
//   t = (-b ± √(b² - 4ac)) / 2a
//
// If the square root in the solution has a real solution (> 0), we hit the sphere.
fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * oc.dot(ray.direction);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let dir = ray.direction.unit();
    let t = 0.5 * (dir.y + 1.0);
    Color((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0))
}

const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: usize = 224;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;

const FOCAL_LENGTH: f64 = 1.0;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - 0.5 * horizontal - 0.5 * vertical - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    write!(stdout, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    for i in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:>3}", i);
        for j in (0..IMAGE_WIDTH).rev() {
            let u = j as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let v = i as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            write!(stdout, "{}", ray_color(&ray))?;
        }
    }
    eprintln!("\nDone.");

    Ok(())
}
