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
        // We also could store the integers inside a single INT
        if orbit.center == center {
            result += count_orbits(orbits, &orbit.orbits, depth + 1);
        }
    }

    result
}

fn find_path<'a>(orbits: &'a Vec<Orbit>, center: &str, goal: &str) -> Option<Vec<&'a str>> {
    for orbit in orbits {
        if orbit.center == center {
            if orbit.orbits == goal {
                return Some(vec![&orbit.orbits, &orbit.center]);
            } else {
                if let Some(mut result) = find_path(orbits, &orbit.orbits, goal) {
                    result.push(&orbit.center);
                    return Some(result);
                }
            }
        }
    }
    None
}

fn count_distance(orbits: &Vec<Orbit>, center: &str, src: &str, dst: &str) -> i32 {
    // Generate a list of parents to `src` and to `dst`
    let mut src_parents = find_path(&orbits, center, src).expect(&format!("no route from {} to {}", center, src));
    let mut dst_parents = find_path(&orbits, center, dst).expect(&format!("no route from {} to {}", center, dst));

    // Determine at which point they start differing. This means all
    // previous parents are common, and the rest different, so we just
    // add how many different there are to count the distance.
    for (i, (s, d)) in src_parents.iter().rev().zip(dst_parents.iter().rev()).enumerate() {
        if s != d {
            // -2 to ignore the both ends
            return (src_parents.len() - i + dst_parents.len() - i - 2) as i32;
        }
    }

    -1
}

fn main() {
    let orbits = read_input();
    println!("{}", count_orbits(&orbits, "COM", 0));
    println!("{}", count_distance(&orbits, "COM", "YOU", "SAN"));
}
