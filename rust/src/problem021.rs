/*
Let d(n) be defined as the sum of proper divisors of n (numbers less than
n which divide evenly into n).

If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair
and each of a and b are called amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22,
44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are
1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.
*/
use common::Factors;


#[test]
fn solve() {
    let d = |n: u64| Factors::new(n).sum::<u64>() - n;
    let mut amicable = vec![false; 10000];
    let mut result = 0;
    for a in 2..amicable.len() as u64 {
        if !amicable[a as usize] {
            let b = d(a);
            if a != b && d(b) == a {
                amicable[a as usize] = true;
                amicable[b as usize] = true;
                result += a + b;
            }
        }
    }
    assert_eq!(result, 31626);
}
