use std::io::{stdin, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left
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

fn fill_path(map: &mut HashMap<(i16, i16), u8>, path: &Vec<Move>, bit: u8) {
    let mut x = 0i16;
    let mut y = 0i16;
    for mov in path {
        match mov.direction {
            Direction::Up => {
                for _ in 0..mov.amount {
                    y += 1;
                    *map.entry((x, y)).or_insert(0) |= bit;
                }
            },
            Direction::Right => {
                for _ in 0..mov.amount {
                    x += 1;
                    *map.entry((x, y)).or_insert(0) |= bit;
                }
            },
            Direction::Down => {
                for _ in 0..mov.amount {
                    y -= 1;
                    *map.entry((x, y)).or_insert(0) |= bit;
                }
            },
            Direction::Left => {
                for _ in 0..mov.amount {
                    x -= 1;
                    *map.entry((x, y)).or_insert(0) |= bit;
                }
            },
        }
    }
}

fn manhattan_from_origin((x, y): (i16, i16)) -> i16 {
    x.abs() + y.abs()
}

fn main() {
    let mut map = HashMap::new();
    let paths = read_inputs();
    let mut bit = 1;
    for path in paths {
        fill_path(&mut map, &path, bit);
        bit <<= 1;
    }

    println!("{}", manhattan_from_origin(*map
        .iter()
        .filter(|(_, bit)| **bit == 0b11)
        .min_by_key(|(pos, _)| manhattan_from_origin(**pos))
        .unwrap()
        .0
    ));
}
