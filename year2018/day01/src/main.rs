use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

fn part1() -> i32 {
    BufReader::new(File::open("input").expect("failed to open"))
        .lines()
        .map(|x| x.expect("failed to read"))
        .map(|x| x.parse::<i32>().expect("malformed input"))
        .sum()
}

fn part2() -> i32 {
    let numbers = BufReader::new(File::open("input").expect("failed to open"))
        .lines()
        .map(|x| x.expect("failed to read"))
        .map(|x| x.parse::<i32>().expect("malformed input"))
        .collect::<Vec<i32>>();

    let mut acc = 0;
    let mut seen = HashSet::with_capacity(numbers.len());
    seen.insert(acc);
    loop {
        for number in numbers.iter() {
            acc += number;
            if !seen.insert(acc) {
                return acc
            }
        }
    }
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
