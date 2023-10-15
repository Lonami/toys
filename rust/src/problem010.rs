/*
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
 */
use common::SievePrimeSeq;

#[test]
fn solve() {
    let result: u64 = SievePrimeSeq::new(2_000_000).sum();
    assert_eq!(result, 142913828922);
}
