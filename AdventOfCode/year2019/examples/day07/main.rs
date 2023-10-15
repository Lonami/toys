use year2019::intcode::Program;
use std::iter::Iterator;

const AMPLIFIER_COUNT: usize = 5;

struct Permutations(Vec<i64>, usize);

impl Permutations {
    fn new(n: usize, start: usize) -> Self {
        Permutations(Vec::with_capacity(n), start)
    }
}

impl Iterator for Permutations {
    type Item = Vec<i64>;

    /// Next lexicographical permutation algorithm
    /// by Project Nayuki, 2017. Public domain.
    /// https://www.nayuki.io/page/next-lexicographical-permutation-algorithm
    fn next(&mut self) -> Option<Self::Item> {
	    if self.0.is_empty() {
            for i in (self.1)..(self.1 + self.0.capacity()) {
                self.0.push(i as i64);
            }
            return Some(self.0.clone());
	    }

	    // Find non-increasing suffix
	    let mut i = self.0.len() - 1;
	    while i > 0 && self.0[i - 1] >= self.0[i] {
		    i -= 1;
	    }
	    if i == 0 {
		    return None;
	    }

	    // Find successor to pivot
	    let mut j = self.0.len() - 1;
	    while self.0[j] <= self.0[i - 1] {
		    j -= 1;
	    }
	    self.0.swap(i - 1, j);

	    // Reverse suffix
	    self.0[i..].reverse();
	    Some(self.0.clone())
    }
}

fn main() {
    let mut program = Program::from_stdin();
    program.save();

    // Part 1 (reusing the same program for all amplifiers through reset)
    println!("{}", Permutations::new(AMPLIFIER_COUNT, 0).map(|phase_settings| {
        let mut last_output = 0i64;
        for phase_setting in phase_settings.iter() {
            program.reset();
            program.set_stdin(vec![*phase_setting, last_output]);
            program.run();
            last_output = program.stdout();
        }
        last_output
    }).max().expect("amplifier count was zero"));

    // Part 2 (each amplifier now needs its own copy because we need to resume)
    program.reset();
    let mut programs = vec![program.clone(); AMPLIFIER_COUNT];
    println!("{}", Permutations::new(AMPLIFIER_COUNT, 5).map(|phase_settings| {
        for (phase_setting, p) in phase_settings.iter().zip(programs.iter_mut()) {
            p.reset();
            p.set_stdin(vec![*phase_setting]);
        }

        // Loop until the last program catches fire, the last output is fed into first
        let mut last_output = 0i64;
        while !programs[programs.len() - 1].on_fire() {
            for p in programs.iter_mut() {
                p.push_input(last_output);
                p.run();
                last_output = p.stdout();
            }
        }
        last_output
    }).max().expect("amplifier count was zero"));
}
