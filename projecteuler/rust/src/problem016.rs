/*
2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2^1000?
*/
fn double(buffer: &mut [u8]) {
    let mut carry = 0;
    for i in 0..buffer.len() {
        buffer[i] = 2 * buffer[i] + carry;
        if buffer[i] < 10 {
            carry = 0;
        } else { 
            carry = buffer[i] / 10;
            buffer[i] %= 10;
        }
    }
}


#[test]
fn solve() {
    let mut buffer = vec![0u8; 1024];
    buffer[0] = 1;
    for _ in 0..1000 {
        double(&mut buffer);
    }
    let result: i32 = buffer.iter().map(|&x| x as i32).sum();
    assert_eq!(result, 1366);
}
