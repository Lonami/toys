mod camera;
mod color;
mod hit;
mod ray;
mod sphere;
mod vec3;

pub use camera::Camera;
pub use color::Color;
pub use hit::{Hit, Hittable, HittableList};
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::Vec3;

use std::io::{self, BufWriter, Write};

fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
    if let Some(hit) = world.hit(ray, 0.0, f64::MAX) {
        return Color(0.5 * (hit.normal + Vec3::new(1.0, 1.0, 1.0)));
    }

    let dir = ray.direction.unit();
    let t = 0.5 * (dir.y + 1.0);
    Color((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0))
}

// Image settings
const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: usize = 224;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

// Camera settings
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;

const FOCAL_LENGTH: f64 = 1.0;

fn main() -> io::Result<()> {
    // Setup
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    write!(stdout, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    for i in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:>3}", i);
        for j in (0..IMAGE_WIDTH).rev() {
            let u = j as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let v = i as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let ray = camera.get_ray(u, v);
            write!(stdout, "{}", ray_color(&ray, &world))?;
        }
    }
    eprintln!("\nDone.");

    Ok(())
}
