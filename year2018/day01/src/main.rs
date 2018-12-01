use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let result = BufReader::new(File::open("input")?)
        .lines()
        .map(|x| x.expect("failed to read"))
        .map(|x| x.parse::<i32>().expect("malformed input"))
        .sum::<i32>();

    println!("{}", result);
    Ok(())
}
