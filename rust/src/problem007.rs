/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
that the 6th prime is 13.

What is the 10 001st prime number?
*/
use common::PrimeSeq;

#[test]
fn solve() {
    let result = PrimeSeq::new().nth(10_000).unwrap();
    assert_eq!(result, 104743);
}
