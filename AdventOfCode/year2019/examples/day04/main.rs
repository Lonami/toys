use std::io::{stdin, Read};

fn read_input_range() -> (i32, i32) {
    let mut buffer = String::new();
    stdin()
        .lock()
        .read_to_string(&mut buffer).expect("error while reading input file");

    let mut input: Vec<i32> = buffer
        .trim_end()
        .split('-')
        .map(|item| item.parse::<i32>().expect("malformed input"))
        .collect();

    input.sort();
    let mut iter = input.into_iter();
    let first = iter.next().expect("empty input");
    let second = iter.next().expect("input needs 2 items");

    // It is a six-digit number.
    // The value is within the range given in your puzzle input.
    (first.max(100000), second.min(999999))
}

fn get_digits(password: i32) -> [i32; 6] {
    [
        (password / 100000) % 10,
        (password / 10000) % 10,
        (password / 1000) % 10,
        (password / 100) % 10,
        (password / 10) % 10,
        (password / 1) % 10
    ]
}

fn is_password_valid(password: i32) -> bool {
    let digits = get_digits(password);

    // Two adjacent digits are the same (like 22 in 122345).
    // Going from left to right, the digits never decrease.
    digits.windows(2).any(|w| w[0] == w[1])
        && digits.windows(2).all(|w| w[0] <= w[1])
}

fn is_second_password_valid(password: i32) -> bool {
    let digits = get_digits(password);

    // Going from left to right, the digits never decrease.
    if !digits.windows(2).all(|w| w[0] <= w[1]) {
        return false;
    }

    let mut pair_count = 0;
    let mut last_digit = digits[0];
    for &digit in digits.iter().skip(1) {
        if digit == last_digit {
            pair_count += 1;
        } else if pair_count == 1 {
            break;
        } else {
            pair_count = 0;
            last_digit = digit;
        }
    }

    pair_count == 1
}

fn main() {
    let (lo, hi) = read_input_range();
    println!("{}", (lo..=hi).filter(|x| is_password_valid(*x)).count());
    println!("{}", (lo..=hi).filter(|x| is_second_password_valid(*x)).count());
}
