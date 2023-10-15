/*
A Pythagorean triplet is a set of three natural numbers, a < b < c,
for which,

    a^2 + b^2 = c^2

For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

// This is arguibly a cleaner solution, but I'm going for functional style
fn _find() -> i32 {
    for a in 100..500i32 {
        for b in a+1..500i32 {
            let c = 1000 - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                return a * b * c;
            }
        }
    }
    return 0;
}

#[test]
fn solve() {
    // Cartesian product for a and b, infer c, filter valid b
    let result = (100..500i32)
        .map(|a| (a, (a+1..500i32)
                  .filter(|b| a.pow(2) + b.pow(2)
                          == (1000 - a - b).pow(2))
                  .next()
        ))
        .filter(|(_, b)| b.is_some())
        .map(|(a, b)| (a, b.unwrap()))
        .map(|(a, b)| a * b * (1000 - a - b))
        .next().unwrap();

    assert_eq!(result, 31875000);
}
