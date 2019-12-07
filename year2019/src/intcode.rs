use std::io::{stdin, Read};

#[derive(Copy, Clone, Debug)]
enum ParameterMode {
    Position = 0,
    Immediate = 1
}

#[derive(Clone, Debug)]
pub struct Program {
    memory: Vec<i32>,
    pc: usize,
    backup: Vec<i32>,
    stdin: Vec<i32>,
    in_pos: usize
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
    /// Build a new program parsing the input from stdin
    pub fn from_stdin() -> Self {
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
            in_pos: 0
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
    }

    /// Change the input noun and verb of the program.
    pub fn set_inputs(&mut self, noun: i32, verb: i32) {
        self.memory[1] = noun;
        self.memory[2] = verb;
    }

    /// Set a vector from which input should be read by the program.
    pub fn set_stdin(&mut self, input: Vec<i32>) {
        self.stdin = input;
    }

    pub fn push_input(&mut self, input: i32) {
        self.stdin.push(input);
    }

    /// Get a mutable reference to the output at (pc + offset)
    fn output(&mut self, offset: usize) -> &mut i32 {
        let pos = self.memory[self.pc + offset] as usize;
        &mut self.memory[pos]
    }

    /// Fetch the value at (pc + offset), based on the given parameter mode.
    fn input(&self, offset: usize, mode: ParameterMode) -> i32 {
        let value = self.memory[self.pc + offset];
        match mode {
            ParameterMode::Position => self.memory[value as usize],
            ParameterMode::Immediate => value
        }
    }

    /// Fetch the latest output value (stdout)
    pub fn stdout(&self) -> i32 {
        self.memory[0]
    }

    /// Operate on a 4-wide instruction (param1, param2, output)
    fn operate(&mut self, operation: impl Fn(i32, i32) -> i32, modes: &[ParameterMode; 3]) -> usize {
        *self.output(3) = operation(self.input(1, modes[0]), self.input(2, modes[1]));
        4
    }

    /// Read an instruction and return its (opcode, parameter modes)
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

    pub fn on_fire(&self) -> bool {
        self.read_ins().0 == 99
    }

    /// Step the program by reading and executing one instruction.
    pub fn step(&mut self) -> bool {
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
                    *self.output(1) = *value;
                    self.in_pos += 1;
                    2
                } else {
                    return false; // halt, we need more input
                }
            },
            4 => { // write
                self.memory[0] = self.input(1, modes[0]);
                2
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
                self.operate(|a, b| (a < b) as i32, &modes)
            },
            8 => { // sete
                self.operate(|a, b| (a == b) as i32, &modes)
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

    /// Run the program until it halts.
    pub fn run(&mut self) {
        while self.step() {
        }
    }
}
