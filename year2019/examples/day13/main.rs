use year2019::intcode::{Program, StepResult};
use std::collections::HashMap;
use std::convert::Into;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HPaddle = 3,
    Ball = 4,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Debug)]
enum OutputState {
    WaitX,
    WaitY,
    WaitTileId,
}

#[derive(Debug)]
struct Game {
    map: HashMap<Position, Tile>,
    pos: Position,
    state: OutputState,
}

impl Into<Tile> for i64 {
    fn into(self) -> Tile {
        match self {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HPaddle,
            4 => Tile::Ball,
            _ => panic!(format!("cannot convert {} into a tile", self)),
        }
    }
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Game {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            pos: Position::new(0, 0),
            state: OutputState::WaitX,
        }
    }

    fn tile(&self, pos: &Position) -> Tile {
        *self.map.get(pos).unwrap_or(&Tile::Empty)
    }

    fn run(&mut self, program: &mut Program) {
        loop {
            match program.step() {
                StepResult::Continue => continue,
                StepResult::NeedInput => continue,
                StepResult::Output(value) => {
                    self.state = match self.state {
                        OutputState::WaitX => {
                            self.pos.x = value as i32;
                            OutputState::WaitY
                        },
                        OutputState::WaitY => {
                            self.pos.y = value as i32;
                            OutputState::WaitTileId
                        },
                        OutputState::WaitTileId => {
                            self.map.insert(self.pos, value.into());
                            OutputState::WaitX
                        },
                    }
                },
                StepResult::CaughtFire => break
            }
        }
    }

    fn remaining_blocks(&self) -> usize {
        self.map.values().filter(|v| **v == Tile::Block).count()
    }
}

impl fmt::Display for Game {
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
                f.write_str(match self.tile(&Position::new(x, y)) {
                    Tile::Empty => " ",
                    Tile::Wall => "█",
                    Tile::Block => "▒",
                    Tile::HPaddle => "━",
                    Tile::Ball => "●",
                })?;
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

fn main() {
    let mut program = Program::from_stdin();
    let mut game = Game::new();
    game.run(&mut program);
    println!("{}", game.remaining_blocks());
}
