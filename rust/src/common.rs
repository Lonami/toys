use std::collections::HashMap;


// Fibonacci Sequence
pub struct Fib {
    a: u64,
    b: u64
}

impl Fib {
    pub fn new() -> Self {
        Self { a: 1, b: 2 }
    }
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.a;
        self.a = self.b;
        self.b += result;
        Some(result)
    }
}

// Prime Numbers
pub fn is_prime(n: u64) -> bool {
    match n {
        n if n < 4 => n >= 2,
        n if n % 2 == 0 => false,
        n if ((n - 5) % 6 != 0) && ((n - 7) % 6 != 0) => false,
		n => {
            (3..)
                .step_by(2)
                .take_while(|&i| i * i <= n)
                .all(|i| n % i != 0)
		}
	}
}

// Prime Generators
// TODO Create a new prime seq that uses cached primes
pub struct PrimeSeq(u64);

impl PrimeSeq {
    pub fn new() -> Self {
        Self { 0: 2 }
    }
}

impl Iterator for PrimeSeq {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(if self.0 == 2 {
            self.0 = 1;
            2
        } else {
            while {
                self.0 += 2;
                !is_prime(self.0)
            } {}
            self.0
        })
    }
}


// Eratosthenes Sieve
pub struct _EratosPrimeSeq {
    q: u64,
    d: HashMap<u64, u64>
}

impl _EratosPrimeSeq {
    pub fn _new() -> Self {
        Self { q: 2, d: HashMap::new() }
    }
}

impl Iterator for _EratosPrimeSeq {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.q == 2 {
            self.q = 3;
            return Some(2)
        }
        loop {
            match self.d.remove(&self.q) {
                Some(p) => {
                    let p2 = p * 2;
                    let mut x = p2 + self.q;
                    while self.d.contains_key(&x) {
                        x += p2;
                    }
                    self.d.insert(x, p);
                    self.q += 2;
                },
                None => {
                    self.d.insert(self.q.pow(2), self.q);
                    self.q += 2;
                    return Some(self.q - 2);
                }
            }
        }
    }
}

// Naive Sieve
pub struct SievePrimeSeq {
    value: usize,
    marked: Vec<bool>
}

impl SievePrimeSeq {
    pub fn new(size: usize) -> Self {
        Self { value: 2, marked: vec![true; size] }
    }
}

impl Iterator for SievePrimeSeq {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value == 2 {
            self.value = 3;
            return Some(2);
        }

        while self.value < self.marked.len() {
            if self.marked[self.value] {
                for i in (self.value..self.marked.len())
                         .step_by(self.value) {
                    self.marked[i] = false;
                }
                return Some(self.value as u64);
            }
            self.value += 2;
        }
        None
    }
}

// Triangular Numbers
pub struct Triangular {
    n: u64,
    i: u64
}

impl Triangular {
    pub fn new() -> Self {
        Self { n: 0, i: 1 }
    }
}

impl Iterator for Triangular {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += self.i;
        self.i += 1;
        Some(self.n)
    }
}

// Factors
pub struct Factors {
    n: u64,
    i: u64,
    pair: Option<u64>
}

impl Factors {
    pub fn new(n: u64) -> Self {
        Factors { n, i: 0, pair: None }
    }

    pub fn count(n: u64) -> u64 {
        let mut result = 0;
        let mut i = 1;
        while i * i < n {
            if n % i == 0 {
                result += 2
            }
            i += 1
        }
        if i * i == n {
            result + 1
        } else {
            result
        }
    }

    pub fn sum(n: u64) -> u64 {
        let mut i = 2;
        let mut result = 1;
        while i * i < n {
            if n % i == 0 {
                result += i + n / i;
            }
            i += 1;
        }
        if i * i == n {
            result += i;
        }
        result
    }
}

impl Iterator for Factors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(p) = self.pair {
            self.pair = None;
            return Some(p);
        }

        while self.i.pow(2) <= self.n {
            self.i += 1;
            if self.n % self.i == 0 {
                let p = self.n / self.i;
                if p != self.i {
                    self.pair = Some(p);
                }
                return Some(self.i);
            }
        }
        None
    }
}

// Collatz Sequence
pub struct Collatz(u64);

impl Collatz {
    pub fn new(n: u64) -> Self {
        Self { 0: n }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        match self.0 {
            0 => None,
            1 => {
                self.0 = 0;
                Some(1)
            }
            n if n % 2 == 0 => {
                self.0 /= 2;
                Some(n)
            }
            n => {
                self.0 = 3 * self.0 + 1;
                Some(n)
            }
        }
    }
}
