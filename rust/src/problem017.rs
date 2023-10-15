/*
If the numbers 1 to 5 are written out in words: one, two, three, four, five,
then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out
in words, how many letters would be used?

Note: do not count spaces or hyphens. For example, 342 (three hundred and
forty-two) contains 23 letters and 115 (one hundred and fifteen) contains
20 letters. The use of "and" when writing out numbers is in compliance with
British usage.
*/
const ZERO_TO_NINETEEN: [&'static str; 20] = [
    "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
    "ten", "eleven", "twelve", "thirteen", "fourteen",
    "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"
];

const TWENTY_TO_NINETY: [&'static str; 8] = [
    "twenty", "thirty", "forty", "fifty",
    "sixty", "seventy", "eighty", "ninety"
];

fn int_to_word(n: i32) -> Option<&'static str> {
    match n {
        n if n < 0 || n > 90 => None,
        n if n < 20 => Some(ZERO_TO_NINETEEN[n as usize]),
        n if n % 10 != 0 => None,
        _ => Some(TWENTY_TO_NINETY[((n - 20) / 10) as usize])
    }
}

fn digit_to_word(n: u8) -> &'static str {
    int_to_word(n as i32).unwrap()
}

fn int_to_digits(mut n: i32) -> [u8; 4] {
    let mut result = [0u8; 4];
    for i in 0..result.len() {
        result[i] = (n % 10) as u8;
        n /= 10;
    }
    result
}

fn to_english_numeral(value: i32) -> String {
    assert!(value > -10_000 && value < 10_000);
    if let Some(word) = int_to_word(value) {
        return String::from(word);
    }

    let mut result = String::new();
    let value = if value < 0 {
        result.push_str("minus ");
        -value
    } else {
        value
    };

    let digits = int_to_digits(value);
    if digits[3] != 0 {
        result.push_str(digit_to_word(digits[3]));
        result.push_str(" thousand ");
    }
    if digits[2] != 0 {
        result.push_str(digit_to_word(digits[2]));
        result.push_str(" hundred ");
    }

    let mut has_and = false;
    if digits[1] != 0 {
        if value > 100 {
            result.push_str("and ");
            has_and = true;
        }
        if digits[1] < 2 {
            result.push_str(int_to_word(value % 100).unwrap());
            return result;
        }
        result.push_str(int_to_word(digits[1] as i32 * 10).unwrap());
        if digits[0] != 0 {
            result.push('-');
        }
    }

    if value == 0 {
        result.push_str(digit_to_word(0));
    } else if digits[0] != 0 {
        if !has_and && digits[1] == 0 && value > 100 {
            result.push_str("and ");
        }
        result.push_str(digit_to_word(digits[0]));
    }

    if *result.as_bytes().last().unwrap() == b' ' {
        let last = result.len() - 1;
        result.truncate(last);
    }
    result
}


#[test]
fn solve() {
    let result: usize = (1..=1000)
        .map(|i| to_english_numeral(i))
        .map(|x| x
             .as_bytes()
             .iter()
             .filter(|&&c| c != b' ' && c != b'-')
             .count())
        .sum();

    assert_eq!(result, 21124);
}
