/*
Euler discovered the remarkable quadratic formula:

    n^2+n+41

It turns out that the formula will produce 40 primes for the consecutive
integer values 0 ≤ n ≤ 39. However, when n = 40, 40^2+40+41 = 40(40+1)+41
is divisible by 41, and certainly when n = 41, 41^2+41+41 is clearly
divisible by 41.

The incredible formula n^2−79n+1601 was discovered, which produces 80
primes for the consecutive values 0 ≤ n ≤ 79. The product of the
coefficients, −79 and 1601, is −126479.

Considering quadratics of the form:

    n^2+an+b, where |a| < 1000 and |b| ≤ 1000

    where |n| is the modulus/absolute value of n
    e.g. |11| = 11 and |−4| = 4

Find the product of the coefficients, a and b, for the quadratic expression
that produces the maximum number of primes for consecutive values of n,
starting with n = 0.
*/
use common::PrimeChecker;


fn f(a: i32, b: i32, n: i32) -> u64 {
    match n * n + a * n + b {
        n if n < 0 => 0, // trying to factor -1 as u64 is bad
        n => n as u64
    }
}


#[test]
fn solve() {
    let mut max_n = 0i32;
    let mut max_ab = 0i32;
    let mut primes = PrimeChecker::new();
    for a in (-999..0i32).rev().step_by(2) {
        for b in (3..1000i32).rev().step_by(2) {
            if !primes.is_prime(b as u64) {
                continue; // n = 0 gives a prime, so b must be one
            }
            let mut n = max_n + 1;
            if !primes.is_prime(f(a, b, n)) {
                continue; // n must be >= before we can consider a/b
            }
            n = 0;
            while primes.is_prime(f(a, b, n)) {
                n += 1;
            }
            if n > max_n {
                max_n = n;
                max_ab = a * b;
            }
        }
    }
    assert_eq!(max_ab, -59231);
}
