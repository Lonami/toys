use std::io::{stdin, Read};

#[derive(Debug)]
struct Program {
    memory: Vec<i32>,
    pc: usize
}

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

        Self { memory, pc: 0 }
    }

    fn set_alarm(&mut self) {
        self.memory[1] = 12;
        self.memory[2] = 2;
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
    program.set_alarm();
    program.run();
    println!("{}", program.read(0));
}
