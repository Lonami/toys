/*
A permutation is an ordered arrangement of objects. For example, 3124 is one
possible permutation of the digits 1, 2, 3 and 4. If all of the permutations
are listed numerically or alphabetically, we call it lexicographic order.
The lexicographic permutations of 0, 1 and 2 are:

    012   021   102   120   201   210

What is the millionth lexicographic permutation of the digits
0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
*/
fn fact(n: i32) -> i32 {
    (2..=n).product()
}

fn permute_n(vector: &mut [i32], n: i32) {
    // Each i'th item permutation has i! permutations below
    let factorial = (0..vector.len() as i32)
        .rev().map(fact).collect::<Vec<i32>>();

    let mut perms = 0;
    let mut index = 0;
    let mut shift = 0;
    while perms != n {
        perms += factorial[index];
        if perms <= n {
            shift += 1;
        }
        if perms >= n {
            if perms > n {
                perms -= factorial[index];
            }

            let new = vector[index + shift];
            for i in ((index + 1)..=(index + shift)).rev() {
                vector[i] = vector[i - 1];
            }
            vector[index] = new;
            shift = 0;

            index += 1;
        }
    }
}


#[test]
fn solve() {
    let mut digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    permute_n(&mut digits, 1_000_000 - 1); // Index-0 based (thus -1)
    let result: u32 = digits.iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| d as u32 * 10_u32.pow(i as u32))
        .sum();

    assert_eq!(result, 2783915460);
}
