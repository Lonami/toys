/*
The sum of the squares of the first ten natural numbers is,
    1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,
    (1 + 2 + ... + 10)^2 = 552 = 3025

Hence the difference between the sum of the squares of the first ten natural
numbers and the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred
natural numbers and the square of the sum.
*/

#[test]
fn solve() {
    let sum_squares = (1..=100).map(|x: u64| x.pow(2)).sum::<u64>();
    let sum_squared = (1..=100).sum::<u64>().pow(2);
    let result: u64 = sum_squared - sum_squares;
    assert_eq!(result, 25164150);
}
