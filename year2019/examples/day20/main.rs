use std::io::{stdin, Read};
use std::fmt;
use std::collections::HashSet;

const MAX_CONCURRENT_FLOOD: usize = 1000;

#[derive(Clone, Copy, Debug)]
enum Tile {
    Open,
    Wall,
    UnconnectedGate(u8),
    Gate(usize),
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Position {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct Maze {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    entrance: usize,
    exit: usize
}

impl Maze {
    fn from_stdin() -> Self {
        // TODO Work without pre-processed inputs
        let mut tiles = Vec::new();
        let mut buffer = Vec::new();
        let mut gates_found = HashSet::new();

        stdin()
            .lock()
            .read_to_end(&mut buffer)
            .expect("failed to read input");

        let line_width = buffer
            .iter()
            .position(|c| *c == b'\n')
            .expect("malformed input") + 1;

        let mut entrance = None;
        let mut exit = None;
        let mut y = 0;
        for (i, c) in buffer.iter().enumerate() {
            match *c {
                b'.' => {
                    tiles.push(Tile::Open);
                },
                b'#' | b' ' => {
                    tiles.push(Tile::Wall);
                },
                b'0' => {
                    entrance = Some(i - y);
                    tiles.push(Tile::Open);
                },
                b'z' => {
                    exit = Some(i - y);
                    tiles.push(Tile::Open);
                }
                b'1'..=b'y' => {
                    gates_found.insert(*c);
                    tiles.push(Tile::UnconnectedGate(*c));
                },
                b'\n' => {
                    y += 1;
                },
                _ => panic!("malformed input")
            }
        }

        let entrance = entrance.unwrap();
        let exit = exit.unwrap();
        let width = line_width - 1;
        let height = y;

        // Connect the gates (replace IDs with indices)
        for gate in gates_found {
            assert_eq!(2, tiles.iter().filter(|t| match t {
                Tile::UnconnectedGate(id) => *id == gate,
                _ => false
            }).count());

            let left = tiles.iter().position(|t| match t {
                Tile::UnconnectedGate(id) => *id == gate,
                _ => false
            }).unwrap();
            let right = tiles.iter().rposition(|t| match t {
                Tile::UnconnectedGate(id) => *id == gate,
                _ => false
            }).unwrap();

            match tiles.get_mut(left).unwrap() {
                gate @ Tile::UnconnectedGate(_) => *gate = Tile::Gate(right),
                _ => unreachable!()
            }
            match tiles.get_mut(right).unwrap() {
                gate @ Tile::UnconnectedGate(_) => *gate = Tile::Gate(left),
                _ => unreachable!()
            }
        }

        Self { tiles, width, height, entrance, exit }
    }

    fn flood_tile(&mut self, flooded: &mut Vec<usize>, warped: &mut Vec<usize>, index: usize) {
        match self.tiles[index] {
            Tile::Open => flooded.push(index),
            Tile::Wall => { },
            Tile::Gate(tp_index) => {
                flooded.push(index);
                warped.push(tp_index);
            },
            Tile::UnconnectedGate(_) => panic!("some gates are not connected!")
        }
    }

    fn flood(&mut self) -> usize {
        let mut flooding = Vec::new();
        let mut new_flood = Vec::new();
        let mut new_warps = Vec::new();

        flooding.push(self.entrance);

        for step in 1.. {
            for i in flooding.iter() {
                let x = i % self.width;
                let y = i / self.width;

                if x > 0 { // Left
                    self.flood_tile(&mut new_flood, &mut new_warps, i - 1);
                }

                if x < self.width - 1 { // Right
                    self.flood_tile(&mut new_flood, &mut new_warps, i + 1);
                }

                if y > 0 { // Up
                    self.flood_tile(&mut new_flood, &mut new_warps, i - self.width);
                }

                if y < self.height - 1 { // Down
                    self.flood_tile(&mut new_flood, &mut new_warps, i + self.width);
                }
            }

            if new_flood.len() > MAX_CONCURRENT_FLOOD {
                panic!("max concurrent flood limit exceeded, probable exponential growth. is the maze open?");
            }

            for i in new_flood.iter() {
                self.tiles[*i] = Tile::Wall;
                if *i == self.exit {
                    return step;
                }
            }

            std::mem::swap(&mut flooding, &mut new_flood); // flooding <- new_flood
            std::mem::swap(&mut new_flood, &mut new_warps); // new_flood <- new warps
            new_warps.clear();
        }

        unreachable!();
    }
}

// Possible solution for part 2:
//
// Each flood remembers its "level" and warping changes the level.
//
// Issues:
// * Determining which gates go "up" and which go "down", which would need to be implemented.
// * Maybe there can be concurrent floods at different levels so we would need a list of lists.

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, row) in self.tiles.chunks(self.width).enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let i = y * self.width + x;
                match tile {
                    Tile::Open => {
                        if i == self.entrance {
                            f.write_str("IN")
                        } else if i == self.exit {
                            f.write_str("OT")
                        } else {
                            f.write_str("  ")
                        }
                    }
                    Tile::Wall => f.write_str("▓▓"),
                    Tile::Gate(index) => write!(f, "{:02}", index % 100),
                    Tile::UnconnectedGate(id) => write!(f, " {}", *id as char),
                }?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut maze = Maze::from_stdin();
    println!("{}", maze.flood());
}
