use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn part1() -> i32 {
    let mut data = BufReader::new(File::open("input").expect("failed to open"))
        .lines()
        .next()
        .expect("empty file")
        .expect("failed to read")
        .into_bytes();

    let mut any = true;
    while any {
        any = false;
        for i in (0..data.len() - 1).rev() {
            if (data[i + 1] as i32 - data[i] as i32).abs() == 32 {
                any = true;
                data.remove(i);
                data.remove(i);
            }
        }
    }

    data.len() as i32
}

fn main() {
    println!("{}", part1());
}
