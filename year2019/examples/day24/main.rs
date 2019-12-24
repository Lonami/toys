use std::io::{stdin, Read};
use std::fmt;
use std::collections::HashMap;

const ERIS_SIZE: usize = 5;

#[derive(Clone, PartialEq, Debug)]
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

struct MultiEris {
    map: Vec<Tile>,
    layers: usize,
    width: usize,
    height: usize,
    lo: usize,
    hi: usize,
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

impl MultiEris {
    fn new(base: &Eris, layers: usize) -> Self {
        let area = base.height * base.width;
        let mut map = vec![Tile::Empty; layers * area];
        let lo = layers / 2;
        let hi = lo;

        map[(lo * area)..((1 + lo) * area)].clone_from_slice(&base.map[..]);
        MultiEris {
            map,
            layers,
            width: base.width,
            height: base.height,
            lo,
            hi,
        }
    }

    #[inline(always)]
    fn infested(&self, layer: usize, y: usize, x: usize) -> usize {
        (self.map[layer * (self.width * self.height) + y * (self.width) + x] == Tile::Bug) as usize
    }

    fn count_live_neighbours(&self, layer: usize, y: usize, x: usize) -> usize {
        let midy = self.height / 2;
        let midx = self.width / 2;
        if y == midy && x == midx {
            // The middle is always empty and never has direct neighbours
            return 0;
        }

        let mut result = 0;

        // Left
        if x == 0 {
            result += self.infested(layer - 1, midy, midx - 1);
        } else if x == midx + 1 && y == midy {
            for i in 0..self.height {
                result += self.infested(layer - 1, i, self.width - 1);
            }   
        } else {
            result += self.infested(layer, y, x - 1);
        }

        // Right
        if x == self.width - 1 {
            result += self.infested(layer - 1, midy, midx + 1);
        } else if x == midx - 1 && y == midy {
            for i in 0..self.height {
                result += self.infested(layer - 1, i, 0);
            }
        } else {
            result += self.infested(layer, y, x + 1);
        }

        // Up
        if y == 0 {
            result += self.infested(layer - 1, midy - 1, midx);
        } else if y == midy + 1 && x == midx {
            for j in 0..self.width {
                result += self.infested(layer - 1, self.height - 1, j);
            }
        } else {
            result += self.infested(layer, y - 1, x);
        }

        // Down
        if y == self.height - 1 {
            result += self.infested(layer - 1, midy, midx + 1);
        } else if y == midy - 1 && x == midx {
            for j in 0..self.width {
                result += self.infested(layer - 1, 0, j);
            }
        } else {
            result += self.infested(layer, y + 1, x);
        }

        result
    }

    fn tick(&mut self) {
        let mut new = Vec::with_capacity(self.map.len());
        for _ in 0..(self.width * self.height) {
            new.push(Tile::Empty);
        }
        for layer in 1..(self.layers - 1) {
            for y in 0..self.height {
                for x in 0..self.width {
                    let neighbours = self.count_live_neighbours(layer, y, x);
                    new.push(match self.map[layer * (self.width * self.height) + y * (self.height) + x] {
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
                    });
                }
            }
        }
        for _ in 0..(self.width * self.height) {
            new.push(Tile::Empty);
        }
        self.map = new;
    }

    fn count_live(&self) -> usize {
        self.map.iter().filter(|tile| **tile == Tile::Bug).count()
    }

    fn layer(&self, layer: i32) -> Eris {
        let area = self.height * self.width;
        let layer = (((self.layers as i32) / 2) + layer) as usize;
        Eris {
            map: self.map[(layer * area)..((1 + layer) * area)].iter().cloned().collect(),
            width: self.width,
            height: self.height,
        }
    }
}

fn main() {
    let mut eris = Eris::from_stdin();
    let mut multi = MultiEris::new(&eris, 200);

    let mut seen = HashMap::new(); // {bio: iteration}
    for i in 0.. {
        if let Some(_) = seen.insert(eris.biodiversity_rating(), i) {
            println!("{}", eris.biodiversity_rating());
            break;
        }
        eris.tick();
    }

    for _ in 0..200 {
        multi.tick();
    }
    println!("{}", multi.count_live());
}
