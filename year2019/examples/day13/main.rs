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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Joystick {
    Neutral = 0,
    Left = -1,
    Right = 1,
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
    ball_pos: Position,
    paddle_pos: Position,
    score: i64,
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
            ball_pos: Position::new(0, 0),
            paddle_pos: Position::new(0, 0),
            score: 0,
            state: OutputState::WaitX,
        }
    }

    fn tile(&self, pos: &Position) -> Tile {
        *self.map.get(pos).unwrap_or(&Tile::Empty)
    }

    fn run(&mut self, program: &mut Program, display: bool) {
        let mut new_score = false;
        while !new_score || self.remaining_blocks() != 0 {
            new_score = false;
            match program.step() {
                StepResult::Continue => continue,
                StepResult::NeedInput => {
                    if self.paddle_pos.x < self.ball_pos.x {
                        program.push_input(Joystick::Right as i64);
                    } else if self.ball_pos.x < self.paddle_pos.x {
                        program.push_input(Joystick::Left as i64);
                    } else {
                        program.push_input(Joystick::Neutral as i64);
                    }
                },
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
                            if self.pos.x == -1 && self.pos.y == 0 {
                                // Segment display score
                                self.score = value;
                                new_score = true;
                            } else {
                                // Tile ID
                                let tile: Tile = value.into();
                                self.map.insert(self.pos, tile);

                                match tile {
                                    Tile::HPaddle => {
                                        self.paddle_pos = self.pos;
                                    },
                                    Tile::Ball => {
                                        self.ball_pos = self.pos;
                                        if display {
                                            println!("{}", self);
                                            std::thread::sleep(std::time::Duration::from_millis(30));
                                        }
                                    },
                                    _ => { }
                                }
                            }
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

        if cfg!(unix) {
            // Reset cursor pos
            f.write_str("\x1b[H")?;
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
        write!(f, "███▓▓▓▒▒░░ SCORE : {: >5} ░░▒▒▓▓▓███", self.score)
    }
}

fn main() {
    let display = if let Some(flag) = std::env::args().skip(1).next() {
        flag == "-d" || flag == "--display"
    } else {
        eprintln!("note: run with --display (or -d) to display the game");
        false
    };

    let mut program = Program::from_stdin();
    let mut game = Game::new();

    program.save();
    game.run(&mut program, false);
    println!("{}", game.remaining_blocks());

    if display && cfg!(unix) {
        // Get rid of cursor, reset screen
        print!("\x1b[?25l\x1b[2J");
    }

    // The most optimal program would just find how the game
    // calculates the score (there seems to be a part in memory
    // with all the blocks around the middle, and another with
    // the scores in the end), and just add them all without
    // playing. But that's not (as) fun.
    program.reset();
    program.set_first_value(2);
    game.run(&mut program, display);

    if display && cfg!(unix) {
        // Bring cursor back, reset screen
        // Ideally this would work with Ctrl+C
        print!("\x1b[?25h\x1b[2J");
    }


    println!("{}", game.score);
}
