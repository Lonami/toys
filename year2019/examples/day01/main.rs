use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};

fn load_inputs<P: AsRef<Path>>(path: P) -> Vec<i32> {
    BufReader::new(File::open(path).expect("failed to read input file"))
        .lines()
        .map(|line| line.expect("error while reading input file"))
        .map(|x| x.parse::<i32>().expect("malformed input"))
        .collect()
}

fn fuel_for(value: &i32) -> i32 {
    (value / 3 - 2).max(0)
}

fn all_fuel_for(value: &i32) -> i32 {
    let mut result = fuel_for(value);
    let mut last = result;
    while last > 0 {
        last = fuel_for(&last);
        result += last;
    }
    result
}

fn part1(inputs: &Vec<i32>) -> i32 {
    inputs.iter().map(fuel_for).sum::<i32>()
}

fn part2(inputs: &Vec<i32>) -> i32 {
    inputs.iter().map(all_fuel_for).sum::<i32>()
}

fn main() {
    let inputs = load_inputs(env::args().skip(1).next().expect("input file path missing"));
    println!("{}", part1(&inputs));
    println!("{}", part2(&inputs));
}
