use year2019::intcode::{Program, StepResult};
use std::convert::Into;

enum Direction {
    North,
    South,
    West,
    East,
    Tumbling
}

struct Position {
    x: i32,
    y: i32,
}

struct Robot {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    pos: Position,
    dir: Direction
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Space,
    Scaffold
}

impl Into<Direction> for u8 {
    fn into(self) -> Direction {
        match self {
            b'^' => Direction::North,
            b'v' => Direction::South,
            b'<' => Direction::West,
            b'>' => Direction::East,
            b'X' => Direction::Tumbling,
            _ => panic!(format!("cannot convert {} into a direction", self)),
        }
    }
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Robot {
    fn new(program: &mut Program) -> Self {
        let mut tiles = Vec::with_capacity(2048);
        let mut width = std::usize::MAX;
        let mut pos = Position::new(0, 0);
        let mut dir = Direction::North;

        loop {
            match program.step() {
                StepResult::Continue => continue,
                StepResult::NeedInput => panic!("cannot handle input during mapgen"),
                StepResult::Output(value) => {
                    match value as u8 {
                        b'.' => tiles.push(Tile::Space),
                        b'#' => tiles.push(Tile::Scaffold),
                        b'\n' => {
                            if width == std::usize::MAX {
                                width = tiles.len();
                            } else if tiles.len() % width != 0 {
                                panic!("malformed output has lines of different lengths");
                            }
                        },
                        b'^' | b'v' | b'<' | b'>' | b'X' => {
                            tiles.push(if value as u8 == b'X' { Tile::Space } else { Tile::Scaffold });
                            pos = Position::new((tiles.len() % width) as i32, (tiles.len() / width) as i32);
                            dir = (value as u8).into();
                        },
                        _ => panic!("malformed output has unknown char")
                    }
                },
                StepResult::CaughtFire => break
            }
        }

        let height = tiles.len() / width;
        Self { tiles, width, height, pos, dir }
    }

    fn alignment_parameter(&self, index: usize) -> usize {
        let (x, y) = (index % self.width, index / self.width);
        if x < 1 || x + 1 >= self.width || y < 1 || y + 1 >= self.height {
            return 0; // looking up or down would be out of bounds
        }

        match self.tiles[index] {
            Tile::Space => 0,
            Tile::Scaffold => {
                let up = self.tiles[x + self.width * (y - 1)];
                let down = self.tiles[x + self.width * (y + 1)];
                let left = self.tiles[x - 1 + self.width * y];
                let right = self.tiles[x + 1 + self.width * y];
                if up == Tile::Scaffold && up == down && up == left && up == right {
                    x * y
                } else {
                    0
                }
            }
        }
    }

    fn sum_alignment_parameters(&self) -> usize {
        (0..self.tiles.len()).map(|i| self.alignment_parameter(i)).sum()
    }
}

fn main() {
    let mut program = Program::from_stdin();
    let mut robot = Robot::new(&mut program);
    println!("{}", robot.sum_alignment_parameters());
}
