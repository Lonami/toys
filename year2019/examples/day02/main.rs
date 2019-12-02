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
