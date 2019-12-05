use year2019::intcode::Program;

const PART_2_GOAL: i32 = 19690720;

fn main() {
    let mut program = Program::new();
    program.save();
    program.set_alarm();
    program.run();
    println!("{}", program.read(0));

    'outer:
    for noun in 0..100 {
        for verb in 0..100 {
            program.reset();
            program.set_inputs(noun, verb);
            program.run();
            if program.read(0) == PART_2_GOAL {
                println!("{}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}
