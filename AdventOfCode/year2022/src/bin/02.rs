use aoc2022::{Error, Result};

use std::fs;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum PlayResult {
    Victory = 6,
    Draw = 3,
    Defeat = 0,
}

#[derive(Debug)]
struct Play {
    opponet: Hand,
    expected: PlayResult,
}

impl Hand {
    fn beats(&self, other: Hand) -> PlayResult {
        match (*self, other) {
            (Hand::Rock, Hand::Rock) => PlayResult::Draw,
            (Hand::Rock, Hand::Paper) => PlayResult::Defeat,
            (Hand::Rock, Hand::Scissors) => PlayResult::Victory,
            (Hand::Paper, Hand::Rock) => PlayResult::Victory,
            (Hand::Paper, Hand::Paper) => PlayResult::Draw,
            (Hand::Paper, Hand::Scissors) => PlayResult::Defeat,
            (Hand::Scissors, Hand::Rock) => PlayResult::Defeat,
            (Hand::Scissors, Hand::Paper) => PlayResult::Victory,
            (Hand::Scissors, Hand::Scissors) => PlayResult::Draw,
        }
    }

    fn score(&self) -> i32 {
        *self as u8 as i32
    }
}

impl PlayResult {
    fn assume_hand(&self) -> Hand {
        match *self {
            PlayResult::Defeat => Hand::Rock,
            PlayResult::Draw => Hand::Paper,
            PlayResult::Victory => Hand::Scissors,
        }
    }

    fn needed_hand(&self, other: Hand) -> Hand {
        match (*self, other) {
            (PlayResult::Draw, Hand::Rock) => Hand::Rock,
            (PlayResult::Defeat, Hand::Paper) => Hand::Rock,
            (PlayResult::Victory, Hand::Scissors) => Hand::Rock,
            (PlayResult::Victory, Hand::Rock) => Hand::Paper,
            (PlayResult::Draw, Hand::Paper) => Hand::Paper,
            (PlayResult::Defeat, Hand::Scissors) => Hand::Paper,
            (PlayResult::Defeat, Hand::Rock) => Hand::Scissors,
            (PlayResult::Victory, Hand::Paper) => Hand::Scissors,
            (PlayResult::Draw, Hand::Scissors) => Hand::Scissors,
        }
    }

    fn score(&self) -> i32 {
        *self as u8 as i32
    }
}

impl Play {
    fn assumed_score(&self) -> i32 {
        self.expected.assume_hand().score()
            + self.expected.assume_hand().beats(self.opponet).score()
    }

    fn score(&self) -> i32 {
        self.expected.needed_hand(self.opponet).score() + self.expected.score()
    }
}

fn parse_hand(chr: u8) -> Result<Hand> {
    match chr {
        b'A' | b'X' => Ok(Hand::Rock),
        b'B' | b'Y' => Ok(Hand::Paper),
        b'C' | b'Z' => Ok(Hand::Scissors),
        _ => Err("bad hand".into()),
    }
}

fn parse_victory_result(chr: u8) -> Result<PlayResult> {
    match chr {
        b'X' => Ok(PlayResult::Defeat),
        b'Y' => Ok(PlayResult::Draw),
        b'Z' => Ok(PlayResult::Victory),
        _ => Err("bad play result".into()),
    }
}

fn parse_input(input: &str) -> Result<Vec<Play>> {
    Ok(input.lines().try_fold(Vec::new(), |mut plays, line| {
        plays.push(Play {
            opponet: parse_hand(line.as_bytes()[0])?,
            expected: parse_victory_result(line.as_bytes()[2])?,
        });
        Ok::<_, Error>(plays)
    })?)
}

fn get_assumed_plan_score(plan: &[Play]) -> i32 {
    plan.iter().map(|play| play.assumed_score()).sum()
}

fn get_plan_score(plan: &[Play]) -> i32 {
    plan.iter().map(|play| play.score()).sum()
}

fn main() -> Result<()> {
    let input = parse_input(&fs::read_to_string("inputs/02/1")?)?;
    println!("Part 1 answer: {}", get_assumed_plan_score(&input));
    println!("Part 2 answer: {}", get_plan_score(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() -> Result<()> {
        let input = parse_input(&fs::read_to_string("inputs/02/1.test")?)?;
        assert_eq!(get_assumed_plan_score(&input), 15);
        Ok(())
    }

    #[test]
    fn example2() -> Result<()> {
        let input = parse_input(&fs::read_to_string("inputs/02/1.test")?)?;
        assert_eq!(get_plan_score(&input), 12);
        Ok(())
    }
}
