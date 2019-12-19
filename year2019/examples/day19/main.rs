use year2019::intcode::{Program, StepResult};

fn count_pulls(program: &mut Program, width: usize, height: usize) -> usize {
    program.save();
    let mut count = 0;
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
                    return count;
                }
            },
            StepResult::Output(value) => {
                count += value as usize;
            },
            StepResult::CaughtFire => {
                program.reset();
            },
        }
    }
}

fn main() {
    let mut program = Program::from_stdin();
    println!("{}", count_pulls(&mut program, 50, 50));
}
