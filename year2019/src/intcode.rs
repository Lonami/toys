use std::io::{stdin, Read};

#[derive(Debug)]
pub struct Program {
    memory: Vec<i32>,
    pc: usize,
    backup: Vec<i32>
}

impl Program {
    pub fn new() -> Self {
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

    pub fn save(&mut self) {
        self.backup = self.memory.clone();
    }

    pub fn reset(&mut self) {
        self.memory = self.backup.clone();
        self.pc = 0;
    }

    pub fn set_alarm(&mut self) {
        self.set_inputs(12, 2);
    }

    pub fn set_inputs(&mut self, noun: i32, verb: i32) {
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

    pub fn read(&self, index: usize) -> i32 {
        self.memory[index]
    }

    pub fn step(&mut self) -> bool {
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

    pub fn run(&mut self) {
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
