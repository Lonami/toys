use std::io::{stdin, Read};

const BASE_PATTERN: [i64; 4] = [0, 1, 0, -1];

fn read_input() -> Vec<u8> {
    let mut buffer = Vec::new();
    stdin()
        .lock()
        .read_to_end(&mut buffer)
        .expect("failed to read input");

    // Trim newline
    while buffer[buffer.len() - 1] < b'0' {
        buffer.pop();
    }

    // Convert to actual digits (none of that ASCII)
    buffer.iter_mut().for_each(|x| *x -= b'0');
    buffer
}

fn fft_phase(digits: &Vec<u8>) -> Vec<u8> {
    // For each `position`, we consider the entire `digits` input.
    //   We have to multiply each `digit` in the input times the cycling pattern.
    //   The pattern repeats each value `position` times (so we can divide by `position` to "make it longer")
    //   Then, the `index + 1` (since we offset the pattern by `1`) is used to index into the pattern.
    //   We take the sum of all of this.
    // We take only the ones digit.
    (0..digits.len())
        .map(|position| {
            (digits
                .iter()
                .enumerate()
                .map(|(index, digit)| {
                    (*digit as i64)
                        * BASE_PATTERN[((index + 1) / (position + 1)) % BASE_PATTERN.len()]
                })
                .sum::<i64>()
                .abs()
                % 10) as u8
        })
        .collect()
}

fn main() {
    let mut digits = read_input();
    for _ in 0..100 {
        digits = fft_phase(&digits);
    }

    println!(
        "{}",
        digits
            .iter()
            .take(8)
            .map(|d| (d + b'0') as char)
            .collect::<String>()
    );
}
