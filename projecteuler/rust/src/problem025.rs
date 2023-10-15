/*
The Fibonacci sequence is defined by the recurrence relation:

    Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.

Hence the first 12 terms will be:

    F1 = 1
    F2 = 1
    F3 = 2
    F4 = 3
    F5 = 5
    F6 = 8
    F7 = 13
    F8 = 21
    F9 = 34
    F10 = 55
    F11 = 89
    F12 = 144

The 12th term, F12, is the first term to contain three digits.

What is the index of the first term in the Fibonacci sequence to contain
1000 digits?
*/


fn add(result: &mut [u8], a: &[u8], b: &[u8]) {
    let mut carry = 0;
    for i in 0..result.len() {
        result[i] = a[i] + b[i] + carry;
        if result[i] > 9 {
            result[i] -= 10;
            carry = 1;
        } else {
            carry = 0;
        }
    }
}


#[test]
fn solve() {
    let mut a = vec![0; 1000];
    let mut b = vec![0; 1000];
    let mut c = vec![0; 1000];
    let mut t = vec![]; // Temporary
    b[0] = 1;
    c[0] = 1;

    let mut i = 2;
    while c[999] == 0 {
        add(&mut a, &b, &c);
        t = a;
        a = b;
        b = c;
        c = t;
        i += 1;
    }

    assert_eq!(i, 4782);
}
