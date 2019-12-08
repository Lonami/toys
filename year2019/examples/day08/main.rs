use std::io::{stdin, Read};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

#[derive(Debug)]
struct Image {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
}

impl Image {
    fn from_stdin(width: usize, height: usize) -> Self {
        let mut buffer = Vec::new();
        stdin()
            .lock()
            .read_to_end(&mut buffer)
            .expect("error while reading input file");

        while buffer[buffer.len() - 1] == b'\n' {
            buffer.pop();
        }

        buffer.iter_mut().for_each(|x| *x -= b'0');
        assert!((buffer.len() % (width * height)) == 0);
        Image {
            pixels: buffer,
            width,
            height,
        }
    }

    fn area(&self) -> usize {
        self.width * self.height
    }

    fn layer_count(&self) -> usize {
        self.pixels.len() / self.area()
    }

    fn layer(&self, index: usize) -> &[u8] {
        &self.pixels[self.area() * index..self.area() * (index + 1)]
    }

    // 0 is black, 1 is white, and 2 is transparent.
    // first layer in front and the last layer in back
    fn compose_layers(&self) -> Vec<u8> {
        (0..self.area())
            .map(|pi| {
                (0..self.layer_count())
                    .map(|li| self.layer(li)[pi])
                    .find(|pixel| *pixel != 2u8)
                    .unwrap_or(2)
            })
            .collect()
    }
}

fn main() {
    let image = Image::from_stdin(WIDTH, HEIGHT);
    let fewest_zero_layer = (0..image.layer_count())
        .map(|index| image.layer(index))
        .min_by_key(|layer| layer.iter().filter(|pixel| **pixel == 0).count())
        .expect("image had no layers");

    println!(
        "{}",
        fewest_zero_layer
            .iter()
            .filter(|pixel| **pixel == 1)
            .count()
            * fewest_zero_layer
                .iter()
                .filter(|pixel| **pixel == 2)
                .count()
    );

    let composed = image.compose_layers();
    for y in 0..image.height {
        for x in 0..image.width {
            print!("{}", if composed[y * image.width + x] == 1 { "██" } else { "  " });
        }
        println!();
    }
}
