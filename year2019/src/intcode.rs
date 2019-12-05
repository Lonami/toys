use std::io::{stdin, Read};

#[derive(Copy, Clone, Debug)]
enum ParameterMode {
    Position = 0,
    Immediate = 1
}

#[derive(Debug)]
pub struct Program {
    memory: Vec<i32>,
    pc: usize,
    backup: Vec<i32>,
    stdin: Vec<i32>,
    in_pos: usize,
    stdout: Vec<i32>
}

impl ParameterMode {
    fn from_i32(value: i32) -> Self {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!(format!("unknown parameter mode {}", value))
        }
    }
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

        Self {
            memory,
            pc: 0,
            backup: vec![],
            stdin: vec![],
            in_pos: 0,
            stdout: vec![]
        }
    }

    pub fn save(&mut self) {
        self.backup = self.memory.clone();
    }

    pub fn reset(&mut self) {
        self.memory = self.backup.clone();
        self.pc = 0;
        self.in_pos = 0;
        self.stdout.clear();
    }

    pub fn set_alarm(&mut self) {
        self.set_inputs(12, 2);
    }

    pub fn set_inputs(&mut self, noun: i32, verb: i32) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }

    pub fn set_stdin(&mut self, input: Vec<i32>) {
        self.stdin = input;
    }

    pub fn get_stdout(&self) -> &Vec<i32> {
        &self.stdout
    }

    fn read_param(&self, offset: usize, mode: ParameterMode) -> i32 {
        match mode {
            ParameterMode::Position => self.memory[self.memory[self.pc + offset] as usize],
            ParameterMode::Immediate => self.memory[self.pc + offset]
        }
    }

    // (left input value, right input value, output position)
    fn get_operands(&self, modes: &[ParameterMode; 3]) -> (i32, i32, usize) {
        (
            self.read_param(1, modes[0]),
            self.read_param(2, modes[1]),
            self.memory[self.pc + 3] as usize
        )
    }

    pub fn read(&self, index: usize) -> i32 {
        self.memory[index]
    }

    fn read_ins(&self) -> (i32, [ParameterMode; 3]) {
        let ins = self.memory[self.pc];
        let opcode = ins % 100;
        let modes = [
            ParameterMode::from_i32((ins / 100) % 10),
            ParameterMode::from_i32((ins / 1000) % 10),
            ParameterMode::from_i32((ins / 10000) % 10),
        ];

        (opcode, modes)
    }

    pub fn step(&mut self) -> bool {
        let (ins, modes) = self.read_ins();
        self.pc += match ins {
            1 => { // add
                let (lin, rin, out) = self.get_operands(&modes);
                self.memory[out] = lin + rin;
                4
            },
            2 => { // mul
                let (lin, rin, out) = self.get_operands(&modes);
                self.memory[out] = lin * rin;
                4
            },
            3 => { // read
                let out = self.memory[self.pc + 1];
                self.memory[out as usize] = self.stdin[self.in_pos];
                self.in_pos += 1;
                2
            },
            4 => { // write
                let lin = self.memory[self.pc + 1];
                self.stdout.push(self.memory[lin as usize]);
                2
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
