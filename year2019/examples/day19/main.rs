use std::collections::VecDeque;
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

fn find_square(mut program: &mut Program, width: usize, height: usize) -> usize {
    // The beam has to be `width * height`, so our deque
    // needs to remember the `width` across `height` rows.
    //
    // The `x` position of the beam at `y + 1` will be ≥ the `x` position at `y`,
    // which means we don't need to bother going all the way from `0`.
    let mut deque = VecDeque::with_capacity(height);

    enum Status {
        WaitingBeam,
        WaitingEnd
    };

    // There's some pesky rows early where the beam is empty. Just skip them.
    for y in 10.. {
        let mut status = Status::WaitingBeam;
        let mut started = 0;
        let mut size = 0;

        // TODO don't always start at 0
        for x in 0.. {
            let value = value_at(&mut program, x, y);
            size += value;
            match status {
                Status::WaitingBeam => {
                    if value == 1 {
                        started = x;
                        status = Status::WaitingEnd;
                    }
                },
                Status::WaitingEnd => {
                    if value == 0 {
                        break;
                    }
                }
            }
        }

        if deque.len() == height {
            let (_, b) = deque.pop_front().unwrap();
            let (a, _) = deque[height - 2];
            if b > a && b - a >= width {
                return a * 10000 + (y - height);
            }
        }
        deque.push_back((started, started + size));
    }

    panic!("exhausted all integers");
}

fn main() {
    let mut program = Program::from_stdin();
    program.save();

    println!("{}", count_pulls(&mut program, 50, 50));
    println!("{}", find_square(&mut program, 100, 100));
}
