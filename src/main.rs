mod aabb;
mod bvh;
mod camera;
mod carton;
mod color;
mod hit;
mod material;
mod perlin;
mod ray;
mod rect;
mod sphere;
mod texture;
mod translate;
mod vec3;

pub use aabb::AABB;
pub use bvh::BvhNode;
pub use camera::Camera;
pub use carton::Carton;
pub use color::Color;
pub use hit::{Hit, Hittable, HittableList};
pub use material::{Dialectric, DiffuseLight, Lambertian, Material, Metal};
pub use perlin::Perlin;
pub use ray::Ray;
pub use rect::{XyRect, XzRect, YzRect};
pub use sphere::{MovingSphere, Sphere};
pub use texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor, Texture};
pub use translate::Translate;
pub use vec3::Vec3;

use oorandom::Rand64;
use std::cell::RefCell;
use std::env;
use std::io::{self, BufWriter, Write};
use std::rc::Rc;

thread_local!(static RNG: RefCell<Rand64> = RefCell::new(Rand64::new(RANDOM_SEED)));

pub fn rand_f64() -> f64 {
    RNG.with(|rng| rng.borrow_mut().rand_float())
}

pub fn rand_range(low: f64, high: f64) -> f64 {
    low + (high - low) * rand_f64()
}

pub fn rand_u64(low: u64, high: u64) -> u64 {
    RNG.with(|rng| rng.borrow_mut().rand_range(low..high))
}

fn ray_color(ray: &Ray, background: &Color, world: &impl Hittable, depth: usize) -> Color {
    if depth == 0 {
        // No more light is gathered.
        return Color::new(0.0, 0.0, 0.0);
    }

    // Use a value close to 0 to avoid the shadow acne problem since floats are not perfect
    if let Some(hit) = world.hit(ray, 0.001, f64::MAX) {
        let emitted = hit.material.emit(hit.u, hit.v, hit.point);
        if let Some((scattered, attenuation)) = hit.material.scatter(ray, &hit) {
            Color(emitted.0 + attenuation.0 * ray_color(&scattered, background, world, depth - 1).0)
        } else {
            emitted
        }
    } else {
        *background
    }
}

const RANDOM_SEED: u128 = 0;

const MAX_DEPTH: usize = 50;

fn random_scene(ball_count: i32) -> HittableList {
    let mut world = HittableList::new();

    let mat_ground = Box::new(Lambertian::textured(Box::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ))));
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
                let material = Box::new(Lambertian::new(Color(
                    Vec3::new_in_range(0.0, 1.0) * Vec3::new_in_range(0.0, 1.0),
                )));
                let center2 = center + Vec3::new(0.0, rand_range(0.0, 0.5), 0.0);
                world.add(Box::new(MovingSphere::new(
                    center, center2, 0.0, 1.0, 0.2, material,
                )));
                continue;
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

    let mat = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat)));

    let mat = Box::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat)));

    world
}

fn two_spheres() -> HittableList {
    let mut world = HittableList::new();

    let new_sphere = |y| {
        Box::new(Sphere::new(
            Vec3::new(0.0, y, 0.0),
            10.0,
            Box::new(Lambertian::textured(Box::new(CheckerTexture::new(
                Color::new(0.2, 0.3, 0.1),
                Color::new(0.9, 0.9, 0.9),
            )))),
        ))
    };

    world.add(new_sphere(-10.0));
    world.add(new_sphere(10.0));

    world
}

fn two_perlin_spheres() -> HittableList {
    let mut world = HittableList::new();

    let new_sphere = |y, size| {
        Box::new(Sphere::new(
            Vec3::new(0.0, y, 0.0),
            size,
            Box::new(Lambertian::textured(Box::new(NoiseTexture::scaled(4.0)))),
        ))
    };

    world.add(new_sphere(-1000.0, 1000.0));
    world.add(new_sphere(2.0, 2.0));

    world
}

fn earth() -> HittableList {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        Box::new(Lambertian::textured(Box::new(ImageTexture::load(
            "img/earthmap.jpg",
        )))),
    )));

    world
}

fn light() -> HittableList {
    let mut world = HittableList::new();

    let new_sphere = |y, size| {
        Box::new(Sphere::new(
            Vec3::new(0.0, y, 0.0),
            size,
            Box::new(Lambertian::textured(Box::new(NoiseTexture::scaled(4.0)))),
        ))
    };

    world.add(new_sphere(-1000.0, 1000.0));
    world.add(new_sphere(2.0, 2.0));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        Box::new(DiffuseLight::new(Color::new(8.0, 8.0, 8.0))),
    )));

    world.add(Box::new(XyRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        Rc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0))),
    )));

    world
}

fn cornell_box() -> HittableList {
    let mut world = HittableList::new();

    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    world.add(Box::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Box::new(YzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Box::new(XzRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    world.add(Box::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Box::new(XzRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Box::new(XyRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    world.add(Box::new(Carton::new(
        Vec3::new(130.0, 0.0, 65.0),
        Vec3::new(295.0, 165.0, 230.0),
        white.clone(),
    )));
    world.add(Box::new(Carton::new(
        Vec3::new(265.0, 0.0, 295.0),
        Vec3::new(430.0, 330.0, 460.0),
        white,
    )));

    world
}

fn main() -> io::Result<()> {
    // Setup
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    let time0 = 0.0;
    let time1 = 1.0;

    let scene;
    let scene = if let Some(s) = env::args().nth(1) {
        scene = s;
        scene.as_ref()
    } else {
        "cornell"
    };

    // World and camera
    let mut background = Color::new(0.7, 0.8, 1.0);
    let mut look_from = Vec3::new(13.0, 2.0, 3.0);
    let mut look_at = Vec3::new(0.0, 0.0, 0.0);
    let mut vfov = 20.0;
    let mut aperture = 0.0;
    let mut samples_per_pixel = 50;
    let mut aspect_ratio = 16.0 / 9.0;
    let mut image_width = 200;

    let world = match scene {
        "random" => {
            aperture = 0.1;
            random_scene(11)
        }
        "2spheres" => two_spheres(),
        "2perlin" => two_perlin_spheres(),
        "earth" => earth(),
        "light" => {
            samples_per_pixel = 400;
            background = Color::new(0.0, 0.0, 0.0);
            look_from = Vec3::new(26.0, 3.0, 6.0);
            look_at = Vec3::new(0.0, 2.0, 0.0);
            light()
        }
        "cornell" => {
            aspect_ratio = 1.0;
            image_width = 200;
            samples_per_pixel = 200;
            background = Color::new(0.0, 0.0, 0.0);
            look_from = Vec3::new(278.0, 278.0, -800.0);
            look_at = Vec3::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            cornell_box()
        }
        _ => {
            eprintln!("unknown scene '{}'; valid scenes are:", scene);
            eprintln!("random, 2spheres, 2perlin, earth, light, cornell (default)");
            std::process::exit(1);
        }
    };

    let world = BvhNode::new(world, time0, time1);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        time0,
        time1,
    );

    write!(stdout, "P6\n{} {}\n255\n", image_width, image_height)?;

    let scale: f64 = 1.0 / samples_per_pixel as f64;
    for i in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {:>3}", i);
        for j in 0..image_width {
            let pixel_color: Vec3 = (0..samples_per_pixel)
                .map(|_| {
                    let u = (rand_f64() + j as f64) / (image_width as f64 - 1.0);
                    let v = (rand_f64() + i as f64) / (image_height as f64 - 1.0);
                    let ray = camera.get_ray(u, v);
                    ray_color(&ray, &background, &world, MAX_DEPTH).0
                })
                .sum();

            // Gama-correct the color for gamma = 2.0 (square root)
            let color = Color::new(
                (scale * pixel_color.x).sqrt(),
                (scale * pixel_color.y).sqrt(),
                (scale * pixel_color.z).sqrt(),
            );

            stdout.write_all(&color.as_bytes())?;
        }
    }
    eprintln!("\nDone.");

    Ok(())
}
