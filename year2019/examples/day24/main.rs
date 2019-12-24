use std::io::{stdin, Read};
use std::fmt;
use std::collections::HashMap;

const ERIS_SIZE: usize = 5;

#[derive(PartialEq, Debug)]
enum Tile {
    Empty,
    Bug,
}

#[derive(Debug)]
struct Eris {
    map: Vec<Tile>,
    width: usize,
    height: usize,
}

impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Self::Empty,
            b'#' => Self::Bug,
            _ => panic!("unknown tile value")
        }
    }
}

impl Eris {
    fn from_stdin() -> Self {
        let mut buffer = Vec::with_capacity(32);
        stdin()
            .lock()
            .read_to_end(&mut buffer).expect("failed to read input");

        let map: Vec<Tile> = buffer
            .into_iter()
            .filter(|b| *b != b'\n')
            .map(|b| b.into())
            .collect();

        assert_eq!(ERIS_SIZE * ERIS_SIZE, map.len());

        Self {
            map,
            width: ERIS_SIZE,
            height: ERIS_SIZE
        }
    }

    fn count_live_neighbours(&self, index: usize) -> usize {
        let mut result = 0;
        let (x, y) = (index % self.width, index / self.width);
        if x >= 1 {
            result += (self.map[index - 1] == Tile::Bug) as usize;
        }
        if x <= self.width - 2 {
            result += (self.map[index + 1] == Tile::Bug) as usize;
        }
        if y >= 1 {
            result += (self.map[index - self.width] == Tile::Bug) as usize;
        }
        if y <= self.height - 2 {
            result += (self.map[index + self.width] == Tile::Bug) as usize;
        }
        result
    }

    fn tick(&mut self) {
        self.map = self.map
            .iter()
            .enumerate()
            .map(|(i, tile)| {
                let neighbours = self.count_live_neighbours(i);
                match tile {
                    Tile::Empty => {
                        if neighbours == 1 || neighbours == 2 {
                            Tile::Bug
                        } else {
                            Tile::Empty
                        }
                    },
                    Tile::Bug => {
                        if neighbours == 1 {
                            Tile::Bug
                        } else {
                            Tile::Empty
                        }
                    }
                }
            })
            .collect();
    }

    fn biodiversity_rating(&self) -> usize {
        self.map
            .iter()
            .rev()
            .fold(0, |id, tile| (id << 1) | (*tile == Tile::Bug) as usize)
    }
}

impl fmt::Display for Eris {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.map.chunks(self.width) {
            for tile in row {
                match tile {
                    Tile::Empty => f.write_str("."),
                    Tile::Bug => f.write_str("#"),
                }?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut eris = Eris::from_stdin();
    let mut seen = HashMap::new(); // {bio: iteration}
    for i in 0.. {
        if let Some(_) = seen.insert(eris.biodiversity_rating(), i) {
            println!("{}", eris.biodiversity_rating());
            break;
        }
        eris.tick();
    }
}
