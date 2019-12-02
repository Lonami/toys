use std::io::{stdin, BufRead};

fn read_inputs() -> Vec<i32> {
    stdin()
        .lock()
        .lines()
        .map(|line| line.expect("error while reading input file"))
        .map(|line| line.parse::<i32>().expect("malformed input"))
        .collect()
}

fn fuel_for(value: &i32) -> i32 {
    (value / 3 - 2).max(0)
}

fn all_fuel_for(value: &i32) -> i32 {
    let mut result = fuel_for(value);
    let mut last = result;
    while last > 0 {
        last = fuel_for(&last);
        result += last;
    }
    result
}

/* Alternatives for the second part

Recursive:
    fn all_fuel_for(value: &i32) -> i32 {
        let result = fuel_for(value);
        if result > 0 {
            result + all_fuel_for(&result)
        } else {
            result
        }
    }

Without imperative iteration:
    fn all_fuel_for(value: &i32) -> i32 {
        let initial_fuel = fuel_for(value);
        (0..).scan((initial_fuel, initial_fuel), |state, _| {
            let next_fuel = fuel_for(&state.1);
            if next_fuel > 0 {
                *state = (state.0 + next_fuel, next_fuel);
                Some(state.0)
            } else {
                None
            }
        }).last().unwrap_or(0)
    }
*/

// sum(m//3-2 for m in map(int, open('input')))
fn part1(inputs: &Vec<i32>) -> i32 {
    inputs.iter().map(fuel_for).sum::<i32>()
}

// sum((lambda f, m: f(f, m))(lambda f, m: 0 if m < 6 else m//3-2+f(f, m//3-2), m) for m in map(int, open('input')))
fn part2(inputs: &Vec<i32>) -> i32 {
    inputs.iter().map(all_fuel_for).sum::<i32>()
}

fn main() {
    let inputs = read_inputs();
    println!("{}", part1(&inputs));
    println!("{}", part2(&inputs));
}
