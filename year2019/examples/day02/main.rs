use std::io::{stdin, Read};

#[derive(Debug)]
struct Program {
    memory: Vec<i32>,
    pc: usize,
    backup: Vec<i32>
}

const PART_2_GOAL: i32 = 19690720;

impl Program {
    fn new() -> Self {
        let mut buffer = String::new();
        stdin()
            .lock()
            .read_to_string(&mut buffer).expect("error while reading input file");

        let memory: Vec<i32> = buffer
            .trim_end()
            .split(',')
            .map(|item| item.trim().parse::<i32>().expect("malformed input"))
            .collect();

        Self { memory, pc: 0, backup: vec![] }
    }

    fn save(&mut self) {
        self.backup = self.memory.clone();
    }

    fn reset(&mut self) {
        self.memory = self.backup.clone();
        self.pc = 0;
    }

    fn set_alarm(&mut self) {
        self.set_inputs(12, 2);
    }

    fn set_inputs(&mut self, noun: i32, verb: i32) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }

    fn get_operands(&self) -> (usize, usize, usize) {
        (
            self.memory[self.pc + 1] as usize,
            self.memory[self.pc + 2] as usize,
            self.memory[self.pc + 3] as usize
        )
    }

    fn read(&self, index: usize) -> i32 {
        self.memory[index]
    }

    fn step(&mut self) -> bool {
        self.pc += match self.memory[self.pc] {
            1 => { // add
                let (lin, rin, out) = self.get_operands();
                self.memory[out] = self.memory[lin] + self.memory[rin];
                4
            },
            2 => { // mul
                let (lin, rin, out) = self.get_operands();
                self.memory[out] = self.memory[lin] * self.memory[rin];
                4
            },
            99 => { // hcf
                return false;
            },
            _ => {
                1
            }
        };

        true
    }

    fn run(&mut self) {
        while self.step() {
        }
    }
}

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
