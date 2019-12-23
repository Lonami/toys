use year2019::intcode::{Program, StepResult};
use std::collections::HashMap;
use std::convert::Into;
use std::fmt;
use std::ops::Add;

// How big is the map?
const RADIUS: i32 = 20;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum DroidStatus {
    HitWall = 0,
    Moved = 1,
    MovedOnOxygen = 2,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Tile {
    Unknown,
    Home,
    Empty,
    Wall,
    Oxygen,
}

// TODO Reuse all these position-direction related things in lib
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Drone {
    map: HashMap<Position, Tile>,
    pos: Position,
    dir: Direction
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

impl Into<DroidStatus> for i64 {
    fn into(self) -> DroidStatus {
        match self {
            0 => DroidStatus::HitWall,
            1 => DroidStatus::Moved,
            2 => DroidStatus::MovedOnOxygen,
            _ => panic!(format!("cannot convert {} into a tile", self)),
        }
    }
}

impl Tile {
    fn is_empty(&self) -> bool {
        match self {
            Tile::Home | Tile::Empty | Tile::Oxygen => true,
            Tile::Unknown | Tile::Wall => false
        }
    }
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<Direction> for Position {
    type Output = Self;

    fn add(self, dir: Direction) -> Self {
        match dir {
            Direction::North => Self { x: self.x, y: self.y - 1 },
            Direction::South => Self { x: self.x, y: self.y + 1 },
            Direction::West => Self { x: self.x - 1, y: self.y },
            Direction::East => Self { x: self.x + 1, y: self.y },
        }
    }
}

impl Drone {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(Position::new(0, 0), Tile::Home);
        Self {
            map,
            pos: Position::new(0, 0),
            dir: Direction::North
        }
    }

    fn tile(&self, pos: &Position) -> Tile {
        *self.map.get(pos).unwrap_or(&Tile::Unknown)
    }

    // Wall follower with right hand seems to work (left hand doesn't, though…)
    fn find_oxygen(&mut self, program: &mut Program, display: bool) {
        let mut found = false;
        loop {
            match program.step() {
                StepResult::Continue => continue,
                StepResult::NeedInput => {
                    // Do we have a wall on our right?
                    self.dir = match self.tile(&(self.pos + self.dir.turn_right())) {
                        Tile::Wall => {
                            // If so, great, keep moving forward
                            self.dir
                        },
                        Tile::Home | Tile::Empty | Tile::Oxygen => {
                            // No, it's empty, follow the wall that just ended
                            self.dir.turn_right()
                        },
                        Tile::Unknown => {
                            // We don't know! It might be empty, so we will try
                            self.dir.turn_right()
                        }
                    };
                    program.push_input(self.dir as i64);
                },
                StepResult::Output(value) => {
                    match value.into() {
                        DroidStatus::HitWall => {
                            // We hit a wall by venturing into the unknown.
                            // Because of this, we want to turn back left.
                            self.map.insert(self.pos + self.dir, Tile::Wall);
                            self.dir = self.dir.turn_left();
                        },
                        DroidStatus::Moved => {
                            // We successfully moved, so we know this tile is empty.
                            self.pos = self.pos + self.dir;
                            if *self.map.entry(self.pos).or_insert(Tile::Empty) == Tile::Home {
                                break;
                            }
                        },
                        DroidStatus::MovedOnOxygen => {
                            // We successfully moved *and* found the oxygen!
                            self.pos = self.pos + self.dir;
                            self.map.insert(self.pos, Tile::Oxygen);
                            found = true;
                        },
                    }
                    if display && !found {
                        // Not found yet, display on every step
                        println!("{}", self);
                        std::thread::sleep(std::time::Duration::from_millis(16));
                    }
                },
                StepResult::CaughtFire => break
            }
        }
    }

    fn find_solution(&mut self, display: bool) -> usize {
        // Flood the dead-ends (those with 3 walls) by placing another wall
        // Then, once all have been flooded, count how many steps must be taken.
        let original = self.map.clone();
        let dirs = [Direction::North, Direction::East, Direction::South, Direction::West];
        let mut dead_ends = Vec::new();
        loop {
            dead_ends.clear();
            for (pos, _) in self.map.iter().filter(|(_, tile)| **tile == Tile::Empty) {
                if dirs.iter().filter(|dir| !self.tile(&(*pos + **dir)).is_empty()).count() == 3 {
                    dead_ends.push(*pos);
                }
            }
            if dead_ends.is_empty() {
                break;
            }

            for dead_end in dead_ends.iter() {
                self.map.insert(*dead_end, Tile::Wall);
            }
            if display {
                println!("{}", self);
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
        };

        let result = self.map.values().filter(|t| match t {
            Tile::Unknown | Tile::Wall => false,
            Tile::Home | Tile::Empty | Tile::Oxygen => true
        }).count() - 1;

        // Restore our original map without the flood (we modify it for display and self.tile)
        self.map = original;

        result
    }

    fn flood_oxygen(&mut self, display: bool) -> usize {
        // Flood the oxygen (those empty with any oxygen nearby) by placing oxygen.
        // Then, once all have been flooded, return how many steps it took.
        let original = self.map.clone();
        let dirs = [Direction::North, Direction::East, Direction::South, Direction::West];
        let mut fillable_ends = Vec::new();
        for step_count in 0.. {
            fillable_ends.clear();
            for (pos, _) in self.map.iter().filter(|(_, tile)| **tile == Tile::Empty) {
                if dirs.iter().any(|dir| self.tile(&(*pos + *dir)) == Tile::Oxygen) {
                    fillable_ends.push(*pos);
                }
            }
            if fillable_ends.is_empty() {
                // Restore our original map without the flood (we modify it for display and self.tile)
                self.map = original;
                return step_count;
            }

            for end in fillable_ends.iter() {
                self.map.insert(*end, Tile::Oxygen);
            }
            if display {
                println!("{}", self);
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
        };

        panic!("the labyrinth is infinite");
    }
}

impl fmt::Display for Drone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if cfg!(unix) {
            // Reset cursor pos
            f.write_str("\x1b[H")?;
        }
        for y in (-RADIUS - 1)..RADIUS {
            for x in (-RADIUS - 1)..RADIUS {
                if x == self.pos.x && y == self.pos.y {
                    f.write_str(match self.dir {
                        Direction::North => "▲",
                        Direction::South => "▼",
                        Direction::West => "◀",
                        Direction::East => "▶",
                    })?;
                } else {
                    let x = match self.tile(&Position::new(x, y)) {
                        Tile::Unknown => "▒",
                        Tile::Home => "H",
                        Tile::Empty => " ",
                        Tile::Wall => "█",
                        Tile::Oxygen => "O"
                    };
                    f.write_str(x)?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn main() {
    let display = if let Some(flag) = std::env::args().skip(1).next() {
        flag == "-d" || flag == "--display"
    } else {
        eprintln!("note: run with --display (or -d) to display the game");
        false
    };

    if display && cfg!(unix) {
        // Reset screen
        print!("\x1b[2J");
    }

    let mut program = Program::from_stdin();
    let mut game = Drone::new();
    game.find_oxygen(&mut program, display);

    println!("{}", game.find_solution(display));
    println!("{}", game.flood_oxygen(display));
}
