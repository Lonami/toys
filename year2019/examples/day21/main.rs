use year2019::intcode::{Program, StepResult};

// Writable boolean registers:
//   T: Temporary
//   J: Whether to Jump at the end
//
// Readonly boolean registers:
//   A: Is there ground one tile away?
//   B: Is there ground two tiles away?
//   C: Is there ground three tiles away?
//   D: Is there ground four tiles away?
//
// Instructions (where Y is writable):
//   AND X Y    : Y &= X
//   OR  X Y    : Y |= X
//   NOT X Y    : Y = !X
//   WALK       : (begins program)
//
// Gaps to jump:
//             1111111
//   01234567890123456
//   #####.###########
//   #####.#..########
//   #####.##.########
//   #####..#.########
//   #####...#########
fn walk_droid(program: &mut Program) -> Option<usize> {
    // NOT C J: If there's no ground three tiles, we want to jump.
    // AND D J: Unless there is no ground four tiles ahead.
    // NOT A T: If there's no ground right ahead of us...
    // OR  T J: ...we definitely want to jump
    let mut lines = "
NOT C J
AND D J
NOT A T
OR T J
WALK".split('\n').skip(1);

    loop {
        match program.step() {
            StepResult::Continue => continue,
            StepResult::NeedInput => {
                if let Some(line) = lines.next() {
                    for c in line.as_bytes() {
                        program.push_input(*c as i32);
                    }
                    program.push_input(b'\n' as i32);
                }
            }
            StepResult::Output(value) => {
                if value > 127 {
                    break Some(value as usize)
                } else {
                    print!("{}", value as u8 as char);
                }
            },
            StepResult::CaughtFire => break None
        }
    }
}

// New Readonly boolean registers:
//   E: Is there ground five tile away?
//   F: Is there ground six tiles away?
//   G: Is there ground seven tiles away?
//   H: Is there ground eight tiles away?
//   I: Is there ground nine tiles away?
//
// Gaps to jump:
//             1111111
//   01234567890123456
//   #####.###########
//   #####.#..########
//   #####.##.########
//   #####..#.########
//   #####...#########
//   #####.##.##..####
//     ^...
//         .
//
//       ^.
//
//         0123456789
//   ......@..........
//   #####.##.##..####
fn run_droid(program: &mut Program) -> Option<usize> {
    let mut lines = "
NOT E J
AND D J
NOT C T
AND T J
NOT A T
OR T J
RUN".split('\n').skip(1);

    loop {
        match program.step() {
            StepResult::Continue => continue,
            StepResult::NeedInput => {
                if let Some(line) = lines.next() {
                    for c in line.as_bytes() {
                        program.push_input(*c as i32);
                    }
                    program.push_input(b'\n' as i32);
                }
            }
            StepResult::Output(value) => {
                if value > 127 {
                    break Some(value as usize)
                } else {
                    print!("{}", value as u8 as char);
                }
            },
            StepResult::CaughtFire => break None
        }
    }
}
fn main() {
    let mut program = Program::from_stdin();
    program.save();
    println!("{}", walk_droid(&mut program).expect("failed to do it"));
    program.reset();
    println!("{}", run_droid(&mut program).expect("failed to run it"));
}
