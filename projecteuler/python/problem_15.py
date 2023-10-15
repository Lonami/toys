def problem_definition():
    return '''Lattice paths

    Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down,
    there are exactly 6 routes to the bottom right corner.

                ->->   ->--   ->--   v---   v---   v---
                ---v   -v->   -v--   ->_>   ->--   v---
                ---v   ---v   -v->   ---v   -v->   ->->

    How many such routes are there through a 20×20 grid?'''


def get_lattice(size):
    size += 1
    grid = [[1] * size for _ in range(size)]
    for i in range(1, size):
        for j in range(1, size):
            grid[i][j] = grid[i - 1][j] + grid[i][j - 1]

    return grid


def print2d(list2d):
    padding = len(str(list2d[-1][-1]))
    for lst in list2d:
        for item in lst:
            print(str(item).rjust(padding), end=' ')
        print()

print2d(get_lattice(20))
