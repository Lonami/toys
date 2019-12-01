use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    println!("{}",
        BufReader::new(
            File::open(
                env::args().skip(1).next().expect("input file path missing")
            ).expect("failed to read input file")
        )
        .lines()
        .map(|line| line.expect("error while reading input file"))
        .map(|x| x.parse::<i32>().expect("malformed input"))
        .map(|mass| mass / 3 - 2)
        .sum::<i32>());
}
