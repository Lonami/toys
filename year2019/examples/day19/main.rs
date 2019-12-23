use std::collections::VecDeque;
use year2019::intcode::{Program, StepResult};

fn value_at(program: &mut Program, x: usize, y: usize) -> usize {
    program.reset();
    loop {
        match program.step() {
            StepResult::Continue => continue,
            StepResult::NeedInput => program.set_stdin(vec![x as i64, y as i64]),
            StepResult::Output(value) => break value as usize,
            StepResult::CaughtFire => panic!("caught fire before output")
        }
    }
}

fn count_pulls(mut program: &mut Program, width: usize, height: usize) -> usize {
    (0..height).map(|y| (0..width).map(|x| value_at(&mut program, x, y)).sum::<usize>()).sum()
}

fn find_square(mut program: &mut Program, width: usize, height: usize) -> usize {
    // The beam has to be `width * height`, so our deque
    // needs to remember the `width` across `height` rows.
    let mut deque = VecDeque::with_capacity(height);
    let mut last_x = 0;

    // There's some pesky rows early where the beam is empty. Just skip them.
    for y in 10.. {
        // The `x` position of the beam at `y + 1` will be ≥ the `x` position at `y`,
        // which means we don't need to bother going all the way from `0`. Use `last_x`
        // to find where the beam starts in this row, and then count how wide it is.
        last_x = (last_x..).filter(|x| value_at(&mut program, *x, y) == 1).next().unwrap();
        let size = 1 + (last_x + 1..).take_while(|x| value_at(&mut program, *x, y) == 1).count();

        if deque.len() == height {
            let (_, b) = deque.pop_front().unwrap();
            let (a, _) = deque[height - 2];
            if b > a && b - a >= width {
                return a * 10000 + (y - height);
            }
        }

        deque.push_back((last_x, last_x + size));
    }

    panic!("exhausted all integers");
}

fn main() {
    let mut program = Program::from_stdin();
    program.save();

    println!("{}", count_pulls(&mut program, 50, 50));
    println!("{}", find_square(&mut program, 100, 100));
}
