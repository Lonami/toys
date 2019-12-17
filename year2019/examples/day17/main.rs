use year2019::intcode::{Program, StepResult};
use std::convert::Into;
use std::ops::Add;
use std::fmt;

const MAX_INPUT: usize = 20;
const MAX_METHODS: usize = 3;

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
    Tumbling
}

#[derive(Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Movement {
    Left,
    Right,
    Forward(usize)
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

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::Tumbling => Direction::Tumbling
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::Tumbling => Direction::Tumbling
        }
    }
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
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, dir: Direction) -> Self {
        match dir {
            Direction::North => Self { x: self.x, y: self.y.wrapping_sub(1) },
            Direction::South => Self { x: self.x, y: self.y.wrapping_add(1) },
            Direction::West => Self { x: self.x.wrapping_sub(1), y: self.y },
            Direction::East => Self { x: self.x.wrapping_add(1), y: self.y },
            Direction::Tumbling => self
        }
    }
}

impl Movement {
    fn char_cost(&self) -> usize {
        match self {
            Movement::Left => 1,
            Movement::Right => 1,
            Movement::Forward(mut n) => {
                let mut cost = 1;
                while n >= 10 {
                    n /= 10;
                    cost += 1;
                }
                cost
            },
        }
    }
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Movement::Left => f.write_str("L"),
            Movement::Right => f.write_str("R"),
            Movement::Forward(n) => write!(f, "{}", n),
        }
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
                            pos = Position::new(tiles.len() % width, tiles.len() / width);
                            dir = (value as u8).into();
                            tiles.push(if value as u8 == b'X' { Tile::Space } else { Tile::Scaffold });
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

    fn determine_methods(&mut self) {
        // We need to walk over all the scaffolds.
        //
        // For this, we will generate a single string of movements,
        // passing through intersections without turning and only
        // turning when a scaffold ends.
        let mut moves = Vec::new();
        while let Some(movement) = self.determine_move() {
            moves.push(movement);
            self.apply_move(movement);
        }

        /// Does the sequence fit within the limit?
        fn is_sequence_valid(seq: &[Movement]) -> bool {
            let commas = seq.len() - 1;
            let move_cost = seq.iter().map(|mov| mov.char_cost()).sum::<usize>();
            (move_cost + commas) <= MAX_INPUT
        }

        /// Find valid sequences at `offset`
        fn find_valid_seqs(moves: &Vec<Movement>, offset: usize) -> Vec<usize> {
            // Half the input will be commas which are unusable to us.
            (2..=(MAX_INPUT / 2))
                .filter(|len| is_sequence_valid(&moves[offset..(offset + len)]))
                .collect()
        }

        for m in moves.iter().take(moves.len() - 1) {
            eprint!("{},", m);
        }
        eprintln!("{}", moves[moves.len() - 1]);
        
        // Of course, the solution is obvious!
        // TODO Instead of doing it by hand find a way to generate this
        /*
        L,12,L,12,R,4,
        R,10,R,6,R,4,R,4,
        L,12,L,12,R,4,
        R,6,L,12,L,12,
        R,10,R,6,R,4,R,4,
        L,12,L,12,R,4,
        R,10,R,6,R,4,R,4,
        R,6,L,12,L,12,
        R,6,L,12,L,12,
        R,10,R,6,R,4,R,4,
        */
    }

    fn can_walk_to(&self, pos: &Position) -> bool {
        if pos.x >= self.width || pos.y >= self.height {
            false
        } else {
            self.tiles[pos.x + self.width * pos.y] == Tile::Scaffold
        }
    }

    fn apply_move(&mut self, movement: Movement) {
        match movement {
            Movement::Left => self.dir = self.dir.turn_left(),
            Movement::Right => self.dir = self.dir.turn_right(),
            Movement::Forward(n) => {
                // TODO Avoid O(n)
                for _ in 0..n {
                    self.pos = self.pos + self.dir;
                }
            }
        }
    }

    fn determine_move(&self) -> Option<Movement> {
        let mut next = self.pos + self.dir;
        if self.can_walk_to(&next) {
            let mut forward = 0;
            while self.can_walk_to(&next) {
                forward += 1;
                next = next + self.dir;
            }
            Some(Movement::Forward(forward))
        } else if self.can_walk_to(&(self.pos + self.dir.turn_left())) {
            Some(Movement::Left)
        } else if self.can_walk_to(&(self.pos + self.dir.turn_right())) {
            Some(Movement::Right)
        } else {
            None
        }
    }
}

fn main() {
    let mut program = Program::from_stdin();
    program.save();

    let mut robot = Robot::new(&mut program);
    println!("{}", robot.sum_alignment_parameters());

    program.reset();
    program.set_first_value(2);
    program.set_stdin(b"A,B,A,C,B,A,B,C,C,B
L,12,L,12,R,4
R,10,R,6,R,4,R,4
R,6,L,12,L,12
y
".iter().map(|b| *b as i32).collect());
    
    program.run();
    println!("{}", program.stdout());
}
