mod camera;
mod color;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec3;

pub use camera::Camera;
pub use color::Color;
pub use hit::{Hit, Hittable, HittableList};
pub use material::{Dialectric, Lambertian, Material, Metal};
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::Vec3;

use oorandom::Rand64;
use std::cell::RefCell;
use std::io::{self, BufWriter, Write};

thread_local!(static RNG: RefCell<Rand64> = RefCell::new(Rand64::new(RANDOM_SEED)));

pub fn rand_f64() -> f64 {
    RNG.with(|rng| rng.borrow_mut().rand_float())
}

fn ray_color(ray: &Ray, world: &impl Hittable, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // Use a value close to 0 to avoid the shadow acne problem since floats are not perfect
    if let Some(hit) = world.hit(ray, 0.001, f64::MAX) {
        if let Some((scattered, attenuation)) = hit.material.scatter(ray, &hit) {
            return Color(attenuation.0 * ray_color(&scattered, world, depth - 1).0);
        } else {
            return Color::new(0.0, 0.0, 0.0);
        }
    }

    let dir = ray.direction.unit();
    let t = 0.5 * (dir.y + 1.0);
    Color((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0))
}

const RANDOM_SEED: u128 = 0;

// Image settings
const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: usize = 200;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

const SAMPLES_PER_PIXEL: usize = 100;
const MAX_DEPTH: usize = 50;

fn random_scene(ball_count: i32) -> HittableList {
    let mut world = HittableList::new();

    let mat_ground = Box::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground,
    )));

    for a in -ball_count..ball_count {
        let a = a as f64;
        for b in -ball_count..ball_count {
            let b = b as f64;

            let mat_prob = rand_f64();
            let center = Vec3::new(a + 0.9 * rand_f64(), 0.2, b + 0.9 * rand_f64());

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() < 0.9 {
                continue;
            }

            let material: Box<dyn Material> = if mat_prob < 0.8 {
                Box::new(Lambertian {
                    albedo: Color(Vec3::new_in_range(0.0, 1.0) * Vec3::new_in_range(0.0, 1.0)),
                })
            } else if mat_prob < 0.95 {
                Box::new(Metal {
                    albedo: Color(Vec3::new_in_range(0.5, 1.0)),
                    fuzz: 0.5 * rand_f64(),
                })
            } else {
                Box::new(Dialectric { ri: 1.5 })
            };

            world.add(Box::new(Sphere::new(center, 0.2, material)));
        }
    }

    let mat = Box::new(Dialectric { ri: 1.5 });
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat)));

    let mat = Box::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat)));

    let mat = Box::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat)));

    world
}

fn main() -> io::Result<()> {
    // Setup
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    // World
    let world = random_scene(0);

    // Camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    write!(stdout, "P6\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    for i in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:>3}", i);
        for j in 0..IMAGE_WIDTH {
            let pixel_color: Vec3 = (0..SAMPLES_PER_PIXEL)
                .map(|_| {
                    let u = (rand_f64() + j as f64) / (IMAGE_WIDTH as f64 - 1.0);
                    let v = (rand_f64() + i as f64) / (IMAGE_HEIGHT as f64 - 1.0);
                    let ray = camera.get_ray(u, v);
                    ray_color(&ray, &world, MAX_DEPTH).0
                })
                .sum();

            // Gama-correct the color for gamma = 2.0 (square root)
            const SCALE: f64 = 1.0 / SAMPLES_PER_PIXEL as f64;
            let color = Color::new(
                (SCALE * pixel_color.x).sqrt(),
                (SCALE * pixel_color.y).sqrt(),
                (SCALE * pixel_color.z).sqrt(),
            );

            stdout.write_all(&color.as_bytes())?;
        }
    }
    eprintln!("\nDone.");

    Ok(())
}
