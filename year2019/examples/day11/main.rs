use year2019::intcode::{Program, StepResult};
use std::collections::HashMap;
use std::convert::Into;
use std::fmt;

#[derive(Clone, Copy, Debug)]
enum PaintColor {
    Black = 0,
    White = 1
}

#[derive(Clone, Copy, Debug)]
enum Rotation {
    Left = 0,
    Right = 1
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Clone, Copy, Debug)]
enum PaintState {
    WaitingColor,
    WaitingRotation
}

#[derive(Debug)]
struct Painting {
    map: HashMap<Position, PaintColor>,
    dir: Direction,
    pos: Position,
    state: PaintState
}

impl Into<PaintColor> for i64 {
    fn into(self) -> PaintColor {
        match self {
            0 => PaintColor::Black,
            1 => PaintColor::White,
            _ => panic!(format!("cannot convert {} into a color", self))
        }
    }
}

impl Into<Rotation> for i64 {
    fn into(self) -> Rotation {
        match self {
            0 => Rotation::Left,
            1 => Rotation::Right,
            _ => panic!(format!("cannot convert {} into a rotation", self))
        }
    }
}

impl Direction {
    fn turn(&mut self, rot: Rotation) {
        *self = match rot {
            Rotation::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down
            },
            Rotation::Right => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up
            }
        }
    }
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn walk_towards(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1
        }
    }
}

impl Painting {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            dir: Direction::Up,
            pos: Position::new(0, 0),
            state: PaintState::WaitingColor
        }
    }

    fn color(&self, pos: &Position) -> PaintColor {
        *self.map.get(pos).unwrap_or(&PaintColor::White)
    }

    fn current_color(&self) -> PaintColor {
        self.color(&self.pos)
    }

    fn set_color(&mut self, color: PaintColor) {
        self.map.insert(self.pos, color);
    }

    fn paint(&mut self, program: &mut Program) {
        loop {
            match program.step() {
                StepResult::Continue => continue,
                StepResult::NeedInput => {
                    program.push_input(self.current_color() as i64);
                },
                StepResult::Output(value) => {
                    self.state = match self.state {
                        PaintState::WaitingColor => {
                            self.set_color(value.into());
                            PaintState::WaitingRotation
                        },
                        PaintState::WaitingRotation => {
                            self.dir.turn(value.into());
                            self.pos.walk_towards(self.dir);
                            PaintState::WaitingColor
                        },
                    }
                },
                StepResult::CaughtFire => break
            }
        }
    }

    fn count_painted(&self) -> usize {
        self.map.len()
    }
}

impl fmt::Display for Painting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Find min-max positions (top-left corner, bottom-right corner)
        let mut x0 = i32::max_value();
        let mut x1 = i32::min_value();
        let mut y0 = i32::max_value();
        let mut y1 = i32::min_value();
        for pos in self.map.keys() {
            x0 = x0.min(pos.x);
            x1 = x1.max(pos.x);
            y0 = y0.min(pos.y);
            y1 = y1.max(pos.y);
        }

        for y in y0..=y1 {
            for x in x0..=x1 {
                f.write_str(match self.color(&Position::new(x, y)) {
                    PaintColor::Black => " ",
                    PaintColor::White => "█"
                })?;
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

fn main() {
    let mut program = Program::from_stdin();
    let mut painting = Painting::new();
    painting.paint(&mut program);
    println!("{}", painting.count_painted());
    println!("{}", painting);
}
