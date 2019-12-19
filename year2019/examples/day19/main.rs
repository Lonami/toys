use std::convert::Into;
use year2019::intcode::{Program, StepResult};

struct Drone {
    forces: Vec<Force>,
}

#[derive(Clone, Copy, PartialEq)]
enum Force {
    Stationary = 0,
    Pulled = 1,
}

impl Into<Force> for i64 {
    fn into(self) -> Force {
        match self {
            0 => Force::Stationary,
            1 => Force::Pulled,
            _ => panic!(format!("cannot convert {} into a force", self)),
        }
    }
}

impl Drone {
    fn new(program: &mut Program, width: usize, height: usize) -> Self {
        program.save();

        let mut forces = Vec::with_capacity(width * height);
        let mut index_it = (0..width * height).into_iter();
        loop {
            match program.step() {
                StepResult::Continue => continue,
                StepResult::NeedInput => {
                    if let Some(index) = index_it.next() {
                        let x = index % width;
                        let y = index / width;
                        program.push_input(x as i32);
                        program.push_input(y as i32);
                    } else {
                        break;
                    }
                },
                StepResult::Output(value) => {
                    forces.push(value.into());
                },
                StepResult::CaughtFire => {
                    program.reset();
                },
            }
        }

        Self { forces }
    }

    fn count_affected_areas(&self) -> usize {
        self.forces.iter().filter(|force| **force == Force::Pulled).count()
    }
}

fn main() {
    let mut program = Program::from_stdin();
    let drone = Drone::new(&mut program, 50, 50);
    println!("{}", drone.count_affected_areas());
}
