use year2019::intcode::Program;

const PART_2_GOAL: i32 = 19690720;

// Part 1 Python one-liner:
// (lambda s, *a: s(s, *a))(lambda s, a, r, m, pc: s(s, a, r, a(m, m[pc + 3], r(m, pc + 1) + r(m, pc + 2)), pc + 4) if m[pc] == 1 else s(s, a, r, a(m, m[pc + 3], r(m, pc + 1) * r(m, pc + 2)), pc + 4) if m[pc] == 2 else m[0] if m[pc] == 99 else s(s, a, r, m, pc + 1), lambda m, i, v: m[:i] + [v] + m[i+1:], lambda m, i: m[m[i]], (lambda m: m[:1] + [12, 2] + m[3:])(list(map(int, open('input').read().split(',')))), 0)
//
// Non-stupid indent and proper names:
// (lambda step, *args: step(step, *args))(lambda step, assign, read, memory, pc:
//     step(step, assign, read, assign(memory, memory[pc + 3], read(memory, pc + 1) + read(memory, pc + 2)), pc + 4)
//         if memory[pc] == 1 else
//     step(step, assign, read, assign(memory, memory[pc + 3], read(memory, pc + 1) * read(memory, pc + 2)), pc + 4)
//         if memory[pc] == 2 else
//     memory[0]
//         if memory[pc] == 99 else
//     step(step, assign, read, memory, pc + 1),
//     lambda m, i, v: m[:i] + [v] + m[i+1:],
//     lambda m, i: m[m[i]],
//     (lambda m: m[:1] + [12, 2] + m[3:])(list(map(int, open('input').read().split(',')))),
//     0
// )

fn main() {
    let mut program = Program::from_stdin();
    program.save();
    program.set_inputs(12, 2);
    program.run();
    println!("{}", program.stdout());

    'outer:
    for noun in 0..100 {
        for verb in 0..100 {
            program.reset();
            program.set_inputs(noun, verb);
            program.run();
            if program.stdout() == PART_2_GOAL {
                println!("{}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}
