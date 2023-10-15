use aoc2022::{Error, Result};

use std::fs;

type Item = u8;
type Rucksack = Vec<Item>;

fn get_priority(item: Item) -> i32 {
    match item {
        b'a'..=b'z' => (item - b'a' + 1) as i32,
        b'A'..=b'Z' => (item - b'A' + 27) as i32,
        _ => panic!("bad item"),
    }
}

fn parse_input(input: &str) -> Result<Vec<Rucksack>> {
    Ok(input.lines().try_fold(Vec::new(), |mut rucksacks, line| {
        rucksacks.push(line.as_bytes().to_vec());
        Ok::<_, Error>(rucksacks)
    })?)
}

fn find_shared_items(rucksacks: &[Rucksack]) -> Vec<Item> {
    rucksacks
        .iter()
        .map(|rucksack| {
            *rucksack[..rucksack.len() / 2]
                .iter()
                .find(|item| rucksack[rucksack.len() / 2..].contains(item))
                .expect("no shared item")
        })
        .collect()
}

fn sum_group_shared_priorities(rucksacks: &[Rucksack], group_size: usize) -> i32 {
    rucksacks
        .chunks_exact(group_size)
        .map(|group| {
            group
                .iter()
                .map(|rucksack| {
                    rucksack
                        .iter()
                        .fold(0u64, |bitset, item| bitset | (1 << get_priority(*item)))
                })
                .fold(u64::MAX, |aggr_bitset, bitset| aggr_bitset & bitset)
                .trailing_zeros() as i32
        })
        .sum()
}

fn main() -> Result<()> {
    let input = parse_input(&fs::read_to_string("inputs/03/1")?)?;
    println!(
        "Part 1 answer: {}",
        find_shared_items(&input)
            .into_iter()
            .map(get_priority)
            .sum::<i32>()
    );
    println!("Part 2 answer: {}", sum_group_shared_priorities(&input, 3));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() -> Result<()> {
        let input = parse_input(&fs::read_to_string("inputs/03/1.test")?)?;
        assert_eq!(
            find_shared_items(&input)
                .into_iter()
                .map(get_priority)
                .sum::<i32>(),
            157
        );
        Ok(())
    }

    #[test]
    fn example2() -> Result<()> {
        let input = parse_input(&fs::read_to_string("inputs/03/1.test")?)?;
        assert_eq!(sum_group_shared_priorities(&input, 3), 70);
        Ok(())
    }
}
