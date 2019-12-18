use std::io::{stdin, BufRead};
use std::fmt;

#[derive(Clone, Copy, Debug)]
enum Tile {
    Open,
    Wall,
    Key(u8),
    Door(u8),
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct TunnelSystem {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    pos: Position,
}

impl From<u8> for Tile {
    fn from(c: u8) -> Self {
        match c {
            b'.' => Self::Open,
            b'#' => Self::Wall,
            b'a'..=b'z' => Self::Key(c - b'a'),
            b'A'..=b'Z' => Self::Door(c - b'A'),
            _ => panic!("cannot convert char into tile")
        }
    }
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

impl TunnelSystem {
    fn from_stdin() -> Self {
        let mut tiles = Vec::new();
        let mut width = 0;
        let mut pos = Position::new(0, 0);
        for (y, line) in stdin()
                .lock()
                .lines()
                .map(|line| line.expect("failed to read input"))
                .enumerate() {
            if width == 0 {
                width = line.len();
            } else if line.len() != width {
                panic!("malformed input has lines of different length");
            }

            for (x, c) in line.as_bytes().iter().enumerate() {
                if *c == b'@' {
                    pos = Position::new(x, y);
                    tiles.push(Tile::Open);
                } else {
                    tiles.push((*c).into());
                }
            }
        }
        tiles.shrink_to_fit();

        let height = tiles.len() / width;
        TunnelSystem { tiles, width, height, pos }
    }

    fn tile(&self, x: usize, y: usize) -> Tile {
        self.tiles[x + self.width * y]
    }

    fn solve(&mut self) -> usize {
        let mut steps = 0;
        'find_keys: loop {
            // Constantly flood from current position until we find the closest key.
            // Upon unlocking a key, we can remove the door (because we can open it).
            let mut flood = vec![false; self.tiles.len()];
            flood[self.pos.x + self.width * self.pos.y] = true;

            let mut new_floods = Vec::new();
            'flooding: for flood_step in 1.. {
                // For every flooded position, try to flood open neighbours.
                // Break as soon as we flood into somewhere with a key.
                new_floods.clear();
                'outer: for (y, row) in flood.chunks(self.width).enumerate() {
                    for (x, _) in row.iter().enumerate().filter(|(_, flooded)| **flooded) {
                        // Try all neighbours of a flooded position.
                        for &(tx, ty) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)].iter() {
                            match self.tile(tx, ty) {
                                Tile::Open if !flood[tx + self.width * ty] => {
                                    new_floods.push((tx, ty))
                                },
                                Tile::Key(key) => {
                                    // We found a key here! Walk towards it and unlock the door.
                                    // TODO this approach of getting the closest key first doesn't work :(
                                    steps += flood_step;
                                    self.pos = Position::new(tx, ty);
                                    self.tiles[tx + self.width * ty] = Tile::Open;
                                    if let Some(door) = self.tiles.iter().position(|tile| match tile {
                                        Tile::Door(n) if *n == key => true,
                                        _ => false
                                    }) {
                                        self.tiles[door] = Tile::Open;
                                    }
                                    break 'flooding;
                                },
                                _ => { }
                            }
                        }
                    }
                }

                if new_floods.is_empty() {
                    // We flooded everything, there's nothing left to flood, and we found no key.
                    // Looks like we're done!
                    break 'find_keys steps;
                } else {
                    // Apply our new floods to the flood map.
                    for (x, y) in new_floods.iter() {
                        flood[x + self.width * y] = true;
                    }
                }
            }
            
            println!("{}", self);
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    }
}

impl fmt::Display for TunnelSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, row) in self.tiles.chunks(self.width).enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if x == self.pos.x && y == self.pos.y {
                    f.write_str("▚▞")?;
                    continue;
                }
                match tile {
                    Tile::Open => f.write_str("  "),
                    Tile::Wall => f.write_str("▓▓"),
                    Tile::Key(n) => write!(f, "▾{}", (n + b'A') as char),
                    Tile::Door(n) => write!(f, "▒{}", (n + b'A') as char),
                }?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut tunnels = TunnelSystem::from_stdin();
    println!("{}", tunnels.solve());
}
