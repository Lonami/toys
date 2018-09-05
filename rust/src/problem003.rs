/*
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/
use common::is_prime;

#[test]
fn solve() {
    let target = 600851475143;
    let result = (0..(target as f64).sqrt() as u64 + 1)
        .rev()
        .filter(|&x| target % x == 0 && is_prime(x))
        .next()
        .unwrap();

    assert_eq!(result, 6857);
}
