/*
2520 is the smallest number that can be divided by each of the numbers from
1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the
numbers from 1 to 20?
*/
use common::PrimeSeq;

#[test]
fn solve() {
    let step = PrimeSeq::new()
        .take_while(|&x| x <= 20)
        .product::<u64>() as usize;

    let result = (step..)
        .step_by(step)
        .filter(|&x| (2..21).all(|i| x % i == 0))
        .next().unwrap();

    assert_eq!(result, 232792560);
}
