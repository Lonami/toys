use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::{HashMap, BTreeMap};
use std::ops::Sub;

#[macro_use]
extern crate text_io;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq, Ord)]
struct Time {
    year: u16,
    month: u8,
    day:   u8,
    hour:  u8,
    min:   u8
}

#[derive(Debug)]
enum Action {
    Shift(i32),
    Sleep,
    Wake
}

impl Time {
    fn mins(&self) -> i32 {
        (31 * 24 * 60) * self.month as i32
        + (24 * 60) * self.day as i32
        + 60 * self.hour as i32
        + self.min as i32
    }
}

impl Sub for Time {
    type Output = i32;

    fn sub(self, other: Time) -> Self::Output {
        self.mins() - other.mins()
    }
}

fn parse_line(line: &str) -> (Time, Action) {
    let year: u16;
    let month: u8;
    let day:   u8;
    let hour:  u8;
    let min:   u8;

    let mut data = line.bytes();
    scan!(data => "[{}-{}-{} {}:{}] ", year, month, day, hour, min);
    let time = Time { year, month, day, hour, min };

    (time, match data.next().expect("invalid input") {
        b'f' => Action::Sleep,
        b'w' => Action::Wake,
        _   => {
            let id: i32;
            scan!(data => "uard #{}", id); 
            Action::Shift(id)
        }
    })
}

fn parse_input() -> BTreeMap<Time, Action> {
    BufReader::new(File::open("input").expect("failed to open"))
        .lines()
        .map(|x| x.expect("failed to read"))
        .map(|x| parse_line(&x))
        .collect()
}

fn part1(input: &BTreeMap<Time, Action>) -> i32 {
    let mut guards = HashMap::new();
    let mut asleep = HashMap::new();

    let mut current_id = None;
    let mut sleep_since = None;

    // Should figure out a nicer solution
    for (time, action) in input.iter() {
        match action {
            Action::Shift(id) => current_id = Some(id),
            Action::Sleep => sleep_since = Some(time.clone()),
            Action::Wake => {
                let id = current_id.expect("no guard shifted in");
                let ss = sleep_since.clone().expect("woke without sleep");
                *guards.entry(id).or_insert(0) += time.clone() - ss.clone();
                asleep.entry(id).or_insert(vec![]).push((ss, time.clone()));
            }
        }
    }

    let id = **guards.iter()
        .max_by_key(|&(_, x)| x)
        .expect("empty list").0;

    let mut minutes = [0; 60];
    for (start, end) in asleep.get(&id).unwrap().iter() {
        let mut i = start.min;
        while i != end.min {
            minutes[i as usize % 60] += 1;
            i += 1;
        }
    }

    let most_asleep = minutes.iter()
        .enumerate()
        .max_by_key(|&(_, x)| x)
        .unwrap()
        .0 as i32;

    id * most_asleep
}

fn part2(input: &BTreeMap<Time, Action>) -> i32 {
    let mut asleep = HashMap::new();

    let mut current_id = None;
    let mut sleep_since = None;

    for (time, action) in input.iter() {
        match action {
            Action::Shift(id) => current_id = Some(id),
            Action::Sleep => sleep_since = Some(time.clone()),
            Action::Wake => {
                let id = current_id.expect("no guard shifted in");
                let ss = sleep_since.clone().expect("woke without sleep");

                let mut minutes = asleep.entry(id).or_insert(vec![0; 60]);
                let mut i = ss.min;
                while i != time.min {
                    minutes[i as usize % 60] += 1;
                    i += 1;
                }
            }
        }
    }

    // Should figure out a nicer solution
    let mut best_id = 0;
    let mut best_min = 0;
    let mut best_count = 0;
    for (&&id, minutes) in asleep.iter() {
        for (minute, &count) in minutes.iter().enumerate() {
            if count > best_count {
                best_id = id;
                best_min = minute;
                best_count = count;
            }
        }
    }

    best_id * best_min as i32
}

fn main() {
    let input = parse_input();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
