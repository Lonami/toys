use std::collections::HashMap;
use std::io::{stdin, BufRead};
use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Planet(i32);

impl std::convert::From<&str> for Planet {
    fn from(string: &str) -> Self {
        let bytes = string.as_bytes();
        Self(i32::from_ne_bytes([
            0u8,
            *bytes.get(0).unwrap_or(&0u8),
            *bytes.get(1).unwrap_or(&0u8),
            *bytes.get(2).unwrap_or(&0u8)
        ]))
    }
}

impl fmt::Display for Planet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0.to_ne_bytes().iter().filter(|b| **b != 0u8) {
            write!(f, "{}", *byte as char)?;
        };
        Ok(())
    }
}

impl fmt::Debug for Planet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Planet({})", self)
    }
}

type OrbitMap = HashMap<Planet, Vec<Planet>>;

fn parse_planet_pair(string: &str) -> (Planet, Planet) {
    let mut it = string.split(')');
    let center = it.next().expect("empty input").into();
    let orbits = it.next().expect("malformed input lacks ')'").into();
    (center, orbits)
}

fn read_input() -> OrbitMap {
    let mut result = OrbitMap::new();
    for (center, orbits) in stdin()
            .lock()
            .lines()
            .map(|line| line.expect("error while reading input file"))
            .map(|line| parse_planet_pair(&line)) {
        result.entry(center).or_insert(vec![]).push(orbits);
    }
    result
}

fn count_orbits(orbits: &OrbitMap, center: Planet, depth: usize) -> usize {
    // We have `1` direct parent plus `(depth - 1)` indirect parents,
    // for a total of `depth` parents. Plus, we need to count the orbits
    // for all of our children (what is orbitting around our center).
    //
    // If the depth is 0, it means we actually don't have parents, so
    // it cancels out with our assumption of one direct parent nicely.
    depth + orbits
        .get(&center)
        .unwrap_or(&vec![])
        .iter()
        .map(|planet| count_orbits(orbits, *planet, depth + 1))
        .sum::<usize>()
}

fn find_path(orbits: &OrbitMap, center: Planet, goal: Planet) -> Option<Vec<Planet>> {
    if let Some(orbitting) = orbits.get(&center) {
        for orbit in orbitting.iter() {
            if *orbit == goal {
                return Some(vec![center, *orbit]);
            } else if let Some(mut result) = find_path(orbits, *orbit, goal) {
                result.push(center);
                return Some(result);
            }
        }
    }
    None
}

fn count_distance(orbits: &OrbitMap, center: Planet, src: Planet, dst: Planet) -> i32 {
    if src == dst {
        return 0;
    }

    // Generate a list of parents to `src` and to `dst`
    let src_parents = find_path(orbits, center, src).expect(&format!("no route from {} to {}", center, src));
    let dst_parents = find_path(orbits, center, dst).expect(&format!("no route from {} to {}", center, dst));

    // Determine at which point they start differing. This means all
    // previous parents are common, and the rest different, so we just
    // add how many different there are to count the distance.
    for (i, (s, d)) in src_parents.iter().rev().zip(dst_parents.iter().rev()).enumerate() {
        if s != d {
            // -2 to ignore the both ends
            return (src_parents.len() - i + dst_parents.len() - i - 2) as i32;
        }
    }

    // We start at the same center and src != dst, it's impossible the paths from
    // center to src and dst don't differ if we found a route (the `expect` above).
    unreachable!();
}

fn main() {
    let com: Planet = "COM".into();
    let you: Planet = "YOU".into();
    let san: Planet = "SAN".into();
    let orbits = read_input();
    println!("{}", count_orbits(&orbits, com, 0));
    println!("{}", count_distance(&orbits, com, you, san));
}
