use std::io::{stdin, BufRead};

#[derive(Debug)]
struct Orbit {
    center: String,
    orbits: String  // `orbits` is in orbit around `center`
}

impl Orbit {
    fn from_str(string: &str) -> Self {
        let mut it = string.split(')');
        let center = it.next().expect("empty input").into();
        let orbits = it.next().expect("malformed input lacks ')'").into();
        Self { center, orbits }
    }
}

fn read_input() -> Vec<Orbit> {
    stdin()
        .lock()
        .lines()
        .map(|line| line.expect("error while reading input file"))
        .map(|line| Orbit::from_str(&line))
        .collect()
}

fn count_orbits(orbits: &Vec<Orbit>, center: &str, depth: i32) -> i32 {
    // We have `1` direct parent plus `(depth - 1)` indirect parents,
    // for a total of `depth` parents. Plus, we need to count the orbits
    // for all of our children (what is orbitting around our center).
    //
    // If the depth is 0, it means we actually don't have parents, so
    // it cancels out with our assumption of one direct parent nicely.
    let mut result = depth;
    for orbit in orbits {
        // TODO this is yelling for a Map
        if orbit.center == center {
            result += count_orbits(orbits, &orbit.orbits, depth + 1);
        }
    }

    result
}

fn main() {
    let orbits = read_input();
    println!("{}", count_orbits(&orbits, "COM", 0));
}
