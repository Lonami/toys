use year2019::intcode::{Program, StepResult};
use std::convert::Into;
use std::ops::Add;
use std::fmt;

const MAX_INPUT: usize = 20;

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

struct Sequence<'a>(&'a [Movement]);

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

struct Solution<'a> {
    /// Ordered indices on when to apply the functions
    indices: Vec<usize>,
    functions: Vec<&'a [Movement]>
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

impl fmt::Display for Sequence<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for mov in self.0.iter().take(self.0.len() - 1) {
            write!(f, "{},", mov)?;
        }
        write!(f, "{}", self.0[self.0.len() - 1])
    }
}

impl fmt::Display for Solution<'_> {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, index) in self.indices.iter().enumerate() {
            f.write_str(&"ABC"[*index..*index + 1])?;
            f.write_str(if i + 1 == self.indices.len() { "\n" } else { "," })?;
        }
        for function in self.functions.iter() {
            Sequence(function).fmt(&mut f)?;
            f.write_str("\n")?;
        }
        f.write_str("n\n")
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

    /// Walk over all scaffolds, and return a list of movements.
    fn walk_path(&mut self) -> Vec<Movement> {
        let mut moves = Vec::new();
        while let Some(movement) = self.determine_move() {
            moves.push(movement);
            self.apply_move(movement);
        }

        moves
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

/// Does the `seq` fit within the character limit?
fn is_sequence_valid(seq: &[Movement]) -> bool {
    let commas = seq.len() - 1;
    let move_cost = seq.iter().map(|mov| mov.char_cost()).sum::<usize>();
    (move_cost + commas) <= MAX_INPUT
}

/// Return all `offset` where this `seq` occurs.
fn find_offsets(moves: &Vec<Movement>, seq: &[Movement]) -> Vec<usize> {
    (0..(moves.len() - seq.len()))
        .filter(|offset| &moves[*offset..(*offset + seq.len())] == seq)
        .collect()
}

/// Return all `seq` that appear twice or more at `offset`, in descending length.
fn find_repeating_seqs<'a>(moves: &'a Vec<Movement>, offset: usize) -> Vec<&'a [Movement]> {
    (1..=(MAX_INPUT / 2))
        .rev()
        .map(|len| &moves[offset..(offset + len)])
        .filter(|seq| is_sequence_valid(seq) && find_offsets(moves, seq).len() > 1)
        .collect()
}

/// Find 3 "functions" (sequences that appear twice or more)
fn find_functions<'a>(moves: &'a Vec<Movement>) -> Solution<'a> {
    for a in find_repeating_seqs(moves, 0) {
        // While the current offset is `a` continue.
        let mut offset_b = 0;
        loop {
            if &moves[offset_b..(offset_b + a.len())] == a {
                offset_b += a.len();
            } else {
                break;
            }
        }

        for b in find_repeating_seqs(moves, offset_b) {
            // While the current offset is `a` or `b` continue.
            let mut offset_c = offset_b;
            loop {
                if &moves[offset_c..(offset_c + a.len())] == a {
                    offset_c += a.len();
                } else if &moves[offset_c..(offset_c + b.len())] == b {
                    offset_c += b.len();
                } else {
                    break;
                }
            }

            for c in find_repeating_seqs(moves, offset_c) {
                // While the current offset is `a`, `b` or `c` continue.
                let mut offset_d = offset_c;
                while offset_d < moves.len() {
                    if &moves[offset_d..(offset_d + a.len())] == a {
                        offset_d += a.len();
                    } else if &moves[offset_d..(offset_d + b.len())] == b {
                        offset_d += b.len();
                    } else if &moves[offset_d..(offset_d + c.len())] == c {
                        offset_d += c.len();
                    } else {
                        break;
                    }
                }

                if offset_d >= moves.len() {
                    // We managed to reach the end, so this is a valid combination!
                    // Now we can return our solution. Calculate which `seq` were used again,
                    // because we haven't been saving that information.
                    let mut indices = Vec::new();
                    let mut offset = 0;
                    while offset < moves.len() {
                        if &moves[offset..(offset + a.len())] == a {
                            indices.push(0);
                            offset += a.len();
                        } else if &moves[offset..(offset + b.len())] == b {
                            indices.push(1);
                            offset += b.len();
                        } else if &moves[offset..(offset + c.len())] == c {
                            indices.push(2);
                            offset += c.len();
                        }
                    }

                    return Solution {
                        indices,
                        functions: vec![a, b, c]
                    };
                }
            }
        }
    }

    panic!("no solution found");
}

fn main() {
    let mut program = Program::from_stdin();
    program.save();

    let mut robot = Robot::new(&mut program);
    println!("{}", robot.sum_alignment_parameters());

    let moves = robot.walk_path();
    let solution = find_functions(&moves);

    program.reset();
    program.set_first_value(2);
    program.set_stdin(solution.to_string().as_bytes().iter().map(|c| *c as i32).collect());
    program.run();
    println!("{}", program.stdout());
}
