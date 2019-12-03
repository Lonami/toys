use std::io::{stdin, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

enum PathIndex {
    First,
    Second
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    amount: i16
}

impl Move {
    fn from_str(string: &str) -> Self {
        Self {
            amount: string[1..].parse::<i16>().expect("malformed input"),
            direction: match string.as_bytes()[0] {
                b'U' => Direction::Up,
                b'R' => Direction::Right,
                b'D' => Direction::Down,
                b'L' => Direction::Left,
                _ => unreachable!()
            }
        }
    }
}

fn read_inputs() -> Vec<Vec<Move>> {
    stdin()
        .lock()
        .lines()
        .map(|line| line.expect("error while reading input file"))
        .map(|line| line.split(',').map(Move::from_str).collect())
        .collect()
}

// Improvements to be made (maybe):
// ? Don't use a HashMap, use a single Vec<(u32, u32)>.
// ? Find the corners of both paths and pick the largest to have that items in the vector.
// * Avoid so much duplication for each direction.
fn fill_path(map: &mut HashMap<(i16, i16), (u32, u32)>, path: &Vec<Move>, index: PathIndex) {
    let mut x = 0i16;
    let mut y = 0i16;
    let mut i = 0u32;
    for mov in path {
        match mov.direction {
            Direction::Up => {
                for _ in 0..mov.amount {
                    y += 1;
                    i += 1;
                    let mut entry = map.entry((x, y)).or_insert((std::u32::MAX, std::u32::MAX));
                    match index {
                        PathIndex::First => entry.0 = entry.0.min(i),
                        PathIndex::Second => entry.1 = entry.1.min(i)
                    }
                }
            },
            Direction::Right => {
                for _ in 0..mov.amount {
                    x += 1;
                    i += 1;
                    let mut entry = map.entry((x, y)).or_insert((std::u32::MAX, std::u32::MAX));
                    match index {
                        PathIndex::First => entry.0 = entry.0.min(i),
                        PathIndex::Second => entry.1 = entry.1.min(i)
                    }
                }
            },
            Direction::Down => {
                for _ in 0..mov.amount {
                    y -= 1;
                    i += 1;
                    let mut entry = map.entry((x, y)).or_insert((std::u32::MAX, std::u32::MAX));
                    match index {
                        PathIndex::First => entry.0 = entry.0.min(i),
                        PathIndex::Second => entry.1 = entry.1.min(i)
                    }
                }
            },
            Direction::Left => {
                for _ in 0..mov.amount {
                    x -= 1;
                    i += 1;
                    let mut entry = map.entry((x, y)).or_insert((std::u32::MAX, std::u32::MAX));
                    match index {
                        PathIndex::First => entry.0 = entry.0.min(i),
                        PathIndex::Second => entry.1 = entry.1.min(i)
                    }
                }
            },
        }
    }
}

fn manhattan_from_origin((x, y): (i16, i16)) -> i16 {
    x.abs() + y.abs()
}

fn main() {
    let mut inputs = read_inputs();
    let second = inputs.pop().expect("invalid empty input");
    let first = inputs.pop().expect("input is missing second line");

    let mut map = HashMap::new();
    fill_path(&mut map, &first, PathIndex::First);
    fill_path(&mut map, &second, PathIndex::Second);

    let intersections: HashMap<(i16, i16), (u32, u32)> = map
        .into_iter()
        .filter(|(_, (dx, dy))| *dx != std::u32::MAX && *dy != std::u32::MAX)
        .collect();

    println!("{}", manhattan_from_origin(*intersections
        .iter()
        .min_by_key(|(pos, _)| manhattan_from_origin(**pos))
        .unwrap()
        .0
    ));

    let (dx, dy) = intersections
        .iter()
        .min_by_key(|(_, (dx, dy))| *dx + *dy)
        .unwrap()
        .1;

    println!("{}", dx + dy);
}
