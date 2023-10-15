use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_input() -> Vec<u8> {
    BufReader::new(File::open("input").expect("failed to open"))
        .lines()
        .next()
        .expect("empty file")
        .expect("failed to read")
        .into_bytes()
}

fn react(mut data: Vec<u8>) -> Vec<u8> {
    let mut any = true;
    while any {
        any = false;
        let mut i = data.len();
        while i > 1 {
            i -= 1;
            if (data[i] as i32 - data[i - 1] as i32).abs() == 32 {
                any = true;
                data.remove(i);
                i -= 1;
                data.remove(i);
            }
        }
    }
    data
}

fn part1(data: &Vec<u8>) -> i32 {
    react(data.clone()).len() as i32
}

fn part2(data: &Vec<u8>) -> i32 {
    (b'a'..=b'z').map(|skip| {
        react(data
              .iter()
              .filter(|&&x| x != skip && x != skip - 32)
              .map(|&x| x)
              .collect()
              ).len() as i32
    }).min().expect("rust ranges are broken")
}

fn main() {
    let input = read_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
