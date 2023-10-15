/*
n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!
*/
fn mul(buffer: &[u8], mut n: u8) -> Vec<u8> {
    let mut result = vec![0; buffer.len()];
    let mut s = 0;
    while n != 0 {
        let m = n % 10;
        if m != 0 {
            let mut carry = 0;
            for i in s..result.len() {
                result[i] += m * buffer[i - s] + carry;
                if result[i] < 10 {
                    carry = 0;
                } else { 
                    carry = result[i] / 10;
                    result[i] %= 10;
                }
            }
        }
        n /= 10;
        s += 1;
    }
    result
}

#[test]
fn solve() {
    let mut buffer = vec![0u8; 512];
    buffer[0] = 1;
    for i in 2..=100 {
        buffer = mul(&buffer, i);
    }
    let result: i32 = buffer.iter().map(|&x| x as i32).sum();
    assert_eq!(result, 648);
}
