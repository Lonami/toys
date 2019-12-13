use std::io::{stdin, Read};

const MEMORY_SIZE: usize = 4096;

#[derive(Copy, Clone, Debug)]
enum ParameterMode {
    Position = 0,
    Immediate = 1,
    Relative = 2
}

#[derive(Copy, Clone, Debug)]
pub enum StepResult {
    Continue,
    NeedInput,
    Output(i64),
    CaughtFire
}

#[derive(Clone, Debug)]
pub struct Program {
    memory: Vec<i64>,
    pc: usize,
    backup: Vec<i64>,
    stdin: Vec<i32>,
    in_pos: usize,
    relative_base: usize,
    stdout: i64
}

impl ParameterMode {
    fn from_i64(value: i64) -> Self {
        match value {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!(format!("unknown parameter mode {}", value))
        }
    }
}

impl Program {
    /// Build a new program parsing the input from stdin
    pub fn from_stdin() -> Self {
        let mut buffer = String::new();
        stdin()
            .lock()
            .read_to_string(&mut buffer).expect("error while reading input file");

        let mut memory = Vec::with_capacity(MEMORY_SIZE);
        memory.extend(buffer
            .trim_end()
            .split(',')
            .map(|item| item.trim().parse::<i64>().expect("malformed input")));

        while memory.len() < MEMORY_SIZE {
            memory.push(0);
        }

        Self {
            memory,
            pc: 0,
            backup: vec![],
            stdin: vec![],
            in_pos: 0,
            relative_base: 0,
            stdout: 0
        }
    }

    /// Save a backup of the current program's memory
    pub fn save(&mut self) {
        self.backup = self.memory.clone();
    }

    /// Reset the program's memory to the backup, setting the program and input counter to 0.
    pub fn reset(&mut self) {
        self.memory = self.backup.clone();
        self.pc = 0;
        self.stdin.clear();
        self.in_pos = 0;
        self.relative_base = 0;
    }

    /// Change the input noun and verb of the program.
    pub fn set_inputs(&mut self, noun: i32, verb: i32) {
        self.memory[1] = noun as i64;
        self.memory[2] = verb as i64;
    }

    /// Set a vector from which input should be read by the program.
    pub fn set_stdin(&mut self, input: Vec<i32>) {
        self.stdin = input;
    }

    pub fn push_input(&mut self, input: i32) {
        self.stdin.push(input);
    }

    /// Get a mutable reference to the output at (pc + offset)
    fn output(&mut self, offset: usize, mode: ParameterMode) -> &mut i64 {
        let value = self.memory[self.pc + offset];
        match mode {
            ParameterMode::Position => &mut self.memory[value as usize],
            ParameterMode::Immediate => panic!("cannot output to immediate"),
            ParameterMode::Relative => &mut self.memory[(self.relative_base as i64 + value) as usize]
        }
    }

    /// Fetch the value at (pc + offset), based on the given parameter mode.
    fn input(&self, offset: usize, mode: ParameterMode) -> i64 {
        let value = self.memory[self.pc + offset];
        match mode {
            ParameterMode::Position => self.memory[value as usize],
            ParameterMode::Immediate => value,
            ParameterMode::Relative => self.memory[(self.relative_base as i64 + value) as usize]
        }
    }

    /// Fetch the latest output value (stdout)
    pub fn stdout(&self) -> i64 {
        self.stdout
    }

    /// Fetch the first value in memory (old stdout)
    pub fn first_value(&self) -> i64 {
        self.memory[0]
    }

    /// Set the first value in memory
    pub fn set_first_value(&mut self, value: i32) {
        self.memory[0] = value as i64;
    }

    /// Operate on a 4-wide instruction (param1, param2, output)
    fn operate(&mut self, operation: impl Fn(i64, i64) -> i64, modes: &[ParameterMode; 3]) -> usize {
        *self.output(3, modes[2]) = operation(self.input(1, modes[0]), self.input(2, modes[1]));
        4
    }

    /// Read an instruction and return its (opcode, parameter modes)
    fn read_ins(&self) -> (i64, [ParameterMode; 3]) {
        let ins = self.memory[self.pc];
        let opcode = ins % 100;
        let modes = [
            ParameterMode::from_i64((ins / 100) % 10),
            ParameterMode::from_i64((ins / 1000) % 10),
            ParameterMode::from_i64((ins / 10000) % 10),
        ];

        (opcode, modes)
    }

    pub fn on_fire(&self) -> bool {
        self.read_ins().0 == 99
    }

    /// Step the program by reading and executing one instruction.
    pub fn step(&mut self) -> StepResult {
        let (ins, modes) = self.read_ins();
        self.pc += match ins {
            1 => { // add
                self.operate(|a, b| a + b, &modes)
            },
            2 => { // mul
                self.operate(|a, b| a * b, &modes)
            },
            3 => { // read
                if let Some(value) = self.stdin.get(self.in_pos) {
                    *self.output(1, modes[0]) = *value as i64;
                    self.in_pos += 1;
                    2
                } else {
                    return StepResult::NeedInput;
                }
            },
            4 => { // write
                self.stdout = self.input(1, modes[0]);
                self.pc += 2;
                return StepResult::Output(self.stdout);
            },
            5 => { // jnz
                if self.input(1, modes[0]) != 0 {
                    self.pc = self.input(2, modes[1]) as usize;
                    0
                } else {
                    3
                }
            },
            6 => { // jz
                if self.input(1, modes[0]) == 0 {
                    self.pc = self.input(2, modes[1]) as usize;
                    0
                } else {
                    3
                }
            },
            7 => { // setl
                self.operate(|a, b| (a < b) as i64, &modes)
            },
            8 => { // sete
                self.operate(|a, b| (a == b) as i64, &modes)
            },
            9 => { // arl (add to relative base, totally made up)
                self.relative_base = (self.relative_base as i64 + self.input(1, modes[0])) as usize;
                2
            }
            99 => { // hcf
                return StepResult::CaughtFire;
            },
            _ => {
                1
            }
        };

        StepResult::Continue
    }

    /// Run the program until it halts or needs input.
    pub fn run(&mut self) {
        loop {
            match self.step() {
                StepResult::Continue | StepResult::Output(_) => continue,
                StepResult::NeedInput | StepResult::CaughtFire => break
            }
        }
    }
}
