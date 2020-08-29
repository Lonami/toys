mod color;
mod vec3;

pub use color::Color;
pub use vec3::Vec3;

use std::io::{self, BufWriter, Write};

const IMAGE_WIDTH: usize = 128;
const IMAGE_HEIGHT: usize = 128;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());

    write!(stdout, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    for i in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:>3}", i);
        for j in (0..IMAGE_WIDTH).rev() {
            let r = i as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let g = j as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let b = 0.25;

            write!(stdout, "{}", Color::new(r, g, b))?;
        }
    }
    eprintln!("\nDone.");

    Ok(())
}
