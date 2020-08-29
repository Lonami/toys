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

use oorandom::Rand64;
use std::io::{self, BufWriter, Write};

fn ray_color(rng: &mut Rand64, ray: &Ray, world: &impl Hittable, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // Use a value close to 0 to avoid the shadow acne problem since floats are not perfect
    if let Some(hit) = world.hit(ray, 0.001, f64::MAX) {
        let target = hit.point + hit.normal + Vec3::new_random_unit(rng);
        return Color(
            0.5 * ray_color(
                rng,
                &Ray::new(hit.point, target - hit.point),
                world,
                depth - 1,
            )
            .0,
        );
    }

    let dir = ray.direction.unit();
    let t = 0.5 * (dir.y + 1.0);
    Color((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0))
}

const RANDOM_SEED: u128 = 0;

// Image settings
const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: usize = 224;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: usize = 50;

fn main() -> io::Result<()> {
    // Setup
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    let mut rng = Rand64::new(RANDOM_SEED);

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
            let pixel_color: Vec3 = (0..SAMPLES_PER_PIXEL)
                .map(|_| {
                    let u = (rng.rand_float() + j as f64) / (IMAGE_WIDTH as f64 - 1.0);
                    let v = (rng.rand_float() + i as f64) / (IMAGE_HEIGHT as f64 - 1.0);
                    let ray = camera.get_ray(u, v);
                    ray_color(&mut rng, &ray, &world, MAX_DEPTH).0
                })
                .sum();

            // Gama-correct the color for gamma = 2.0 (square root)
            const SCALE: f64 = 1.0 / SAMPLES_PER_PIXEL as f64;
            let color = Color::new(
                (SCALE * pixel_color.x).sqrt(),
                (SCALE * pixel_color.y).sqrt(),
                (SCALE * pixel_color.z).sqrt(),
            );

            write!(stdout, "{}", color)?;
        }
    }
    eprintln!("\nDone.");

    Ok(())
}
