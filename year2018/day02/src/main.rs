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

fn part2() -> String {
    let data = BufReader::new(File::open("input").expect("failed to open"))
        .lines()
        .map(|x| x.expect("failed to read").into_bytes())
        .collect::<Vec<Vec<u8>>>();

    let mut besti = 0;
    let mut bestj = 0;
    let mut bestv = 0;
    for i in 0..data.len() {
        let left = &data[i];
        for j in (i + 1)..data.len() {
            let right = &data[j];
            let mut valid = 0;
            for k in 0..left.len() {
                if left[k] == right[k] {
                    valid += 1;
                }
            }
            if valid > bestv {
                besti = i;
                bestj = j;
                bestv = valid;
            }
        }
    }

    let mut result = String::new();
    let left = &data[besti];
    let right = &data[bestj];
    for i in 0..left.len() {
        if left[i] == right[i] {
            result.push(left[i] as char);
        }
    }
    result
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
