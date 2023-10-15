/*
Starting in the top left corner of a 2×2 grid, and only being able to move
to the right and down, there are exactly 6 routes to the bottom right corner.

    ->->   ->--   ->--   v---   v---   v---
    ---v   -v->   -v--   ->_>   ->--   v---
    ---v   ---v   -v->   ---v   -v->   ->->

How many such routes are there through a 20×20 grid?
*/

fn lattice(n: usize) -> u64 {
    let n = n + 1;
    let mut grid = vec![vec![1; n]; n];
    for i in 1..n {
        for j in 1..n {
            grid[i][j] = grid[i - 1][j] + grid[i][j - 1];
        }
    }
    grid[n - 1][n - 1]
}

#[test]
fn solve() {
    let result = lattice(20);
    assert_eq!(result, 137846528820);
}
