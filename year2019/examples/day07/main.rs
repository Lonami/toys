use year2019::intcode::Program;
use std::iter::Iterator;

const AMPLIFIER_COUNT: usize = 5;

struct Permutations(Vec<i32>);

impl Permutations {
    fn new(n: usize) -> Self {
        Permutations(Vec::with_capacity(n))
    }
}

impl Iterator for Permutations {
    type Item = Vec<i32>;

    /// Next lexicographical permutation algorithm
    /// by Project Nayuki, 2017. Public domain.
    /// https://www.nayuki.io/page/next-lexicographical-permutation-algorithm
    fn next(&mut self) -> Option<Self::Item> {
	    if self.0.is_empty() {
            for i in 0..self.0.capacity() {
                self.0.push(i as i32);
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
    println!("{}", Permutations::new(AMPLIFIER_COUNT).map(|phase_settings| {
        let mut last_output = 0;
        for phase_setting in phase_settings.iter() {
            program.reset();
            program.set_stdin(vec![*phase_setting, last_output]);
            program.run();
            last_output = program.stdout();
        }
        last_output
    }).max().expect("amplifier count was zero"));
}
