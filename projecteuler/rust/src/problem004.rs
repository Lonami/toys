/*
A palindromic number reads the same both ways. The largest palindrome made
from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/
fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    let n = s.as_bytes();
    (0..n.len() / 2).all(|x: usize| n[x] == n[n.len() - x - 1])
}

fn find() -> u64 {
    let mut best = 0;
    for a in 100..1000 {
        for b in 100..1000 {
            if a * b > best && is_palindrome(a * b) {
                best = a * b;
            }
        }
    }
    best
}

#[test]
fn solve() {
    let result = find();
    assert_eq!(result, 906609);
}
