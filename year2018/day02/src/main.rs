use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn part1() -> i32 {
    let mut twos = 0;
    let mut thrs = 0;

    let file = BufReader::new(File::open("input").expect("failed to open"));
    for line in file.lines().map(|x| x.expect("failed to read")) {
        let mut freq = [0; 26];
        for chr in line.into_bytes() {
            let chr = chr - b'a';
            assert!(chr < 26u8, "invalid input");
            freq[chr as usize] += 1;
        }
        let mut two = false;
        let mut thr = false;
        for frq in freq.iter() {
            match frq {
                2 => two = true,
                3 => thr = true,
                _ => {}
            }
        }
        if two { twos += 1 }
        if thr { thrs += 1 }
    }
    twos * thrs
}

fn main() {
    println!("{}", part1());
}
