use std::io::{stdin, BufRead};
use std::collections::HashMap;
use std::fmt;

struct Chemical {
    quantity: i64,
    name: u64
}

struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical
}

// Output name -> Reaction triggering it
type ReactionMap = HashMap<u64, Reaction>;

// Chemical name -> Quantity needed for it
type NeedMap = HashMap<u64, i64>;

impl Chemical {
    fn new_raw(quantity: i64, name: u64) -> Self {
        Self { quantity, name }
    }

    fn new(quantity: i64, name: &str) -> Self {
        Self::new_raw(quantity, u64::from_str_radix(name, 36).expect("invalid chemical name"))
    }

    fn from_str(string: &str) -> Self {
        let mut it = string.trim().split(' ');
        let quantity = it.next().expect("empty string").parse().expect("left-side must be a number");
        let name = it.next().expect("input must contain a space");
        Self::new(quantity, name)
    }
}

impl fmt::Display for Chemical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.quantity)?;

        let mut chars = Vec::with_capacity(8);
        let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut n = self.name;
        while n >= 36 {
            let i = (n % 36) as usize;
            n /= 36;
            chars.push(&digits[i..i + 1]);
        }
        chars.push(&digits[n as usize..n as usize + 1]);

        for c in chars.iter().rev() {
            f.write_str(c)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Chemical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Reaction {
    fn from_line(line: &str) -> Self {
        let mut it = line.split("=>");
        let inputs = it.next().expect("empty string").split(',').map(|string| Chemical::from_str(string)).collect();
        let output = Chemical::from_str(it.next().expect("input must contain =>"));
        Reaction { inputs, output }
    }
}

impl fmt::Display for Reaction {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut it = self.inputs.iter();
        if let Some(reaction) = it.next() {
            reaction.fmt(&mut f)?;
        }
        for reaction in it {
            write!(f, ", {}", reaction)?;
        }
        write!(f, " => {}", self.output)
    }
}

impl fmt::Debug for Reaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

fn read_input() -> ReactionMap {
    stdin()
        .lock()
        .lines()
        .map(|line| line.expect("failed to read input"))
        .map(|line| Reaction::from_line(&line))
        .map(|reaction| (reaction.output.name, reaction))
        .collect()
}

/// Solve the needs in `need_map` with the auxiliary `new_needs` (to avoid allocations),
/// based on the known `reactions`.
fn solve_needs(reactions: &ReactionMap, need_map: &mut NeedMap, new_needs: &mut NeedMap) -> i64 {
    // `need_map`: For `name`, how much `quantity` left do we need?
    // We might have more than enough, in which case it's negative.
    // Since the `need_map` will be reused, this "more than enough"
    // is necessary (and it indicates "we have extra, maybe don't generate more").
    //
    // `new_needs`:  We will need an auxiliary new need map because we can't
    // update `need_map` while we iterate it.
    //
    // (We only do this so we can do `ore.name`)
    let ore = Chemical::new(0, "ORE");
    loop {
        // Is all we need ore? If that's the case, break the loop because we are done.
        if need_map.iter().all(|(name, need)| if *name == ore.name { *need > 0 } else { *need <= 0 }) {
            // Remove the ore need from the map (because it is no longer needed, we just consumed it)
            break need_map.remove(&ore.name).unwrap_or(0);
        }

        // We will have a clean set of new needs
        new_needs.clear();

        // Go over all our needs, and every time we satisfy one, decrease how much left we need
        for (name, need) in need_map.iter_mut() {
            if *name != ore.name && *need > 0 {
                // We still need more for this. What reaction produces this?
                let reaction = &reactions[name];

                // This reaction can produce `quantity`, and we have `need` units.
                // How many times do we need to react?
                let times = (*need + (reaction.output.quantity - 1)) / (reaction.output.quantity);

                // React that many `times`
                *need -= reaction.output.quantity * times;
                for input in reaction.inputs.iter() {
                    *new_needs.entry(input.name).or_insert(0) += input.quantity * times;
                }
            }
        }

        // We have new needs, update our need_map with them
        for (name, need) in new_needs.iter() {
            *need_map.entry(*name).or_insert(0) += need;
        }
    }
}

fn needed_ore(reactions: &ReactionMap, goal: &Chemical) -> i64 {
    let mut need_map: HashMap<u64, i64> = HashMap::with_capacity(reactions.len());
    let mut new_needs: HashMap<u64, i64> = HashMap::with_capacity(reactions.len());

    need_map.insert(goal.name, goal.quantity);
    solve_needs(&reactions, &mut need_map, &mut new_needs)
}

// This is REALLY slow
fn produce_fuel(reactions: &ReactionMap, mut remaining_ore: i64) -> i64 {
    let mut produced_fuel = 0;

    // (We only do this so we can do `fuel.name`)
    let fuel = Chemical::new(1, "FUEL");

    let mut need_map: HashMap<u64, i64> = HashMap::with_capacity(reactions.len());
    let mut new_needs: HashMap<u64, i64> = HashMap::with_capacity(reactions.len());
    while remaining_ore > 0 {
        // TODO better initial estimate
        need_map.insert(fuel.name, 1);
        produced_fuel += 1;
        remaining_ore -= solve_needs(&reactions, &mut need_map, &mut new_needs);
    }

    if remaining_ore < 0 {
        // Physics don't allow negative quantity, so we actually produced one fuel less
        produced_fuel - 1
    } else {
        // Ore left is 0, we produced just enough
        produced_fuel
    }
}

fn main() {
    let reactions = read_input();
    println!("{}", needed_ore(&reactions, &Chemical::new(1, "FUEL")));
    println!("{}", produce_fuel(&reactions, 1000000000000));
}
