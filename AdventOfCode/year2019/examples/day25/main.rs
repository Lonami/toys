use std::io::{stdin, BufRead};
use year2019::intcode::{Program, StepResult};

fn main() {
    let mut program = Program::from_file("inputs/day25/input");
    program.save();
    
    // north, south, east, west
    // take <item>
    // drop <item>
    // inv
    let mut line = String::new();
    loop {
        match program.step() {
            StepResult::Continue => continue,
            StepResult::NeedInput => {
                stdin()
                    .lock()
                    .read_line(&mut line);

                for b in line.as_bytes() {
                    program.push_input(*b as i64);
                }
            }
            StepResult::Output(value) => {
                print!("{}", value as u8 as char);
            },
            StepResult::CaughtFire => break,
        }
    }
}
