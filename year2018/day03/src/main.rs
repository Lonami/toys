use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[macro_use]
extern crate text_io;

#[derive(Debug)]
struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    id: usize
}

fn parse_line(line: &str) -> Rect {
    let x: usize;
    let y: usize;
    let w: usize;
    let h: usize;
    let id: usize;
    scan!(line.bytes() => "#{} @ {},{}: {}x{}", id, x, y, w, h);
    Rect { x, y, w, h, id }
}

fn part1() -> i32 {
    let mut grid = vec![vec![None::<bool>; 1024]; 1024];
    let data = BufReader::new(File::open("input").expect("failed to open"))
        .lines()
        .map(|x| x.expect("failed to read"))
        .map(|x| parse_line(&x));

    for rect in data {
        for y in rect.y..rect.y + rect.h {
            for x in rect.x..rect.x + rect.w {
                grid[y][x] = match grid[y][x] {
                    None => Some(false),
                    _    => Some(true)
                };
            }
        }
    }

    grid.iter()
        .map(|row| row.iter()
                      .map(|x| match x {
                               Some(true) => 1,
                               _          => 0
                           })
                      .sum::<i32>())
        .sum()
}

fn main() {
    println!("{}", part1());
}
