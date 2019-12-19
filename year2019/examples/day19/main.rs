use year2019::intcode::{Program, StepResult};

fn value_at(program: &mut Program, x: usize, y: usize) -> usize {
    program.reset();
    loop {
        match program.step() {
            StepResult::Continue => continue,
            StepResult::NeedInput => program.set_stdin(vec![x as i32, y as i32]),
            StepResult::Output(value) => break value as usize,
            StepResult::CaughtFire => panic!("caught fire before output")
        }
    }
}

fn count_pulls(mut program: &mut Program, width: usize, height: usize) -> usize {
    (0..height).map(|y| (0..width).map(|x| value_at(&mut program, x, y)).sum::<usize>()).sum()
}

fn main() {
    let mut program = Program::from_stdin();
    program.save();

    println!("{}", count_pulls(&mut program, 50, 50));
}
