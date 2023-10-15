use std::io::{stdin, Read};

const BASE_PATTERN: [i64; 4] = [0, 1, 0, -1];
const REAL_SIGNAL_MULTIPLIER: usize = 10000;

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

fn late_fft_phase(digits: &Vec<u8>) -> Vec<u8> {
    // We start with all the remaining digits being multiplied by 1 (so it's their sum).
    // Every next digit just has the previous ones as 0, which is why we constantly subtract.
    // TODO This can probably be made more fancy with a fold of some sort.
    let mut result = Vec::with_capacity(digits.len());
    let mut sum: usize = digits.iter().map(|digit| *digit as usize).sum();
    for d in digits {
        result.push((sum % 10) as u8);
        sum -= *d as usize;
    }
    result
}

fn show_signal(signal: &Vec<u8>) {
    println!(
        "{}",
        signal
            .iter()
            .take(8)
            .map(|d| (d + b'0') as char)
            .collect::<String>()
    );
}

fn main() {
    let original_signal = read_input();
    let mut signal = original_signal.clone();
    for _ in 0..100 {
        signal = fft_phase(&signal);
    }
    show_signal(&signal);

    let signal = original_signal.clone();
    let mut offset = 0;
    for digit in signal.iter().take(7) {
        offset = (offset * 10) + *digit as usize;
    }

    // Because our solution is at `offset` we can `skip` all the initial values,
    // since the pattern would be `0` for these digits and thus never contribute.
    let mut real_signal: Vec<u8> = signal
        .iter()
        .cycle()
        .take(signal.len() * REAL_SIGNAL_MULTIPLIER)
        .skip(offset)
        .map(|d| *d)
        .collect();

    // We will have `offset` ones for the first item, then `offset + 1` and so on.
    // If the offset is larger than our remaining signal, that means we just have
    // to add the digits (because they all multiply by one).
    assert!(real_signal.len() < offset);
    for _ in 0..100 {
        real_signal = late_fft_phase(&real_signal);
    }
    show_signal(&real_signal);
}
