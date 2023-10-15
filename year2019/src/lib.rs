pub mod intcode;

pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }

    if a == 0 || b == 0 {
        return a.max(b).max(1);
    }

    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    a
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / (gcd(a as i32, b as i32) as i64)
}
