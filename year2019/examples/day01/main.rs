use std::io::{stdin, BufRead};

fn read_inputs() -> Vec<i32> {
    stdin()
        .lock()
        .lines()
        .map(|line| line.expect("error while reading input file"))
        .map(|line| line.parse::<i32>().expect("malformed input"))
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
    let inputs = read_inputs();
    println!("{}", part1(&inputs));
    println!("{}", part2(&inputs));
}
