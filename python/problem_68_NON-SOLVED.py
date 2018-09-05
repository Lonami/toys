from itertools import permutations


def problem_definition():
    return '''Consider the following "magic" 3-gon ring, filled with the numbers 1 to 6,
    and each line adding to nine.


                                   (4)
                                      \.
                                       (3)
                                      ./ \.
                                    (1)---(2)---(6)
                                   ./
                                 (5)


    Working clockwise, and starting from the group of three with the numerically lowest external node
    (4,3,2 in this example), each solution can be described uniquely. For example, the above solution
    can be described by the set: 4,3,2; 6,2,1; 5,1,3.

    It is possible to complete the ring with four different totals: 9, 10, 11, and 12.
    There are eight solutions in total.
    Total	Solution Set
    9       4,2,3; 5,3,1; 6,1,2
    9       4,3,2; 6,2,1; 5,1,3
    10      2,3,5; 4,5,1; 6,1,3
    10      2,5,3; 6,3,1; 4,1,5
    11      1,4,6; 3,6,2; 5,2,4
    11      1,6,4; 5,4,2; 3,2,6
    12      1,5,6; 2,6,4; 3,4,5
    12      1,6,5; 3,5,4; 2,4,6

    By concatenating each group it is possible to form 9-digit strings; the maximum string for a 3-gon
    ring is 432621513.

    Using the numbers 1 to 10, and depending on arrangements, it is possible to form 16- and 17-digit
    strings. What is the maximum 16-digit string for a "magic" 5-gon ring?


                                 ( )_
                                     \.         ( )
                                     _( )_    ./
                                   ./     \.  |
                                _( )        ( )
                              ./   \.      ./
                            ( )     |      |
                                    ( )--( )--( )
                                      \.
                                       |
                                       ( )
    '''

# See resources/problem_68.svg for understanding this matrix
# Here it looks like:
# Enabled/disabled     Line 1     Line 2     Line 3     Line 4     Line 5
# 00101                00x01      0010x      00101      00101      00101
# 11110                11x10      111x0      11110      1x110      xxx10
# 01110                01x10      01x10      0xxx0      0x110      01110
# 01000                01000      01000      01000      0x000      01000


def get_line(grid, i):
    """Gets the line from the given grid"""
    if i == 1:
        return tuple([grid[0][2], grid[1][2], grid[2][2]])
    elif i == 2:
        return tuple([grid[0][4], grid[1][3], grid[2][2]])
    elif i == 3:
        return tuple([grid[2][1], grid[2][2], grid[2][3]])
    elif i == 4:
        return tuple([grid[1][1], grid[2][1], grid[3][1]])
    elif i == 5:
        return tuple([grid[1][0], grid[1][1], grid[1][2]])


def set_line(grid, i, values):
    """Sets the line i in the given grid with the given values"""
    if i == 1:
        grid[0][2] = values[0]
        grid[1][2] = values[1]
        grid[2][2] = values[2]

    elif i == 2:
        grid[0][4] = values[0]
        grid[1][3] = values[1]
        grid[2][2] = values[2]

    elif i == 3:
        grid[2][1] = values[0]
        grid[2][2] = values[1]
        grid[2][3] = values[2]

    elif i == 4:
        grid[1][1] = values[0]
        grid[2][1] = values[1]
        grid[3][1] = values[2]

    elif i == 5:
        grid[1][0] = values[0]
        grid[1][1] = values[1]
        grid[1][2] = values[2]


# Line 1 will overwrite no previous lines
# Line 2 will overwrite line 1 in [2][2] at [2]
# Line 3 will overwrite line 1 in [2][2] at [1], line 2 in [2][2] at [1]
# Line 4 will overwrite line 3 in [2][1] at [1]
# Line 5 will overwrite line 1 in [1][2] at [2], line 4 in [1][1] at [1]
def get_wont_overwrite_mask(i):
    """Returns a boolean mask which indicates what values for line i will NOT overwrite previous lines"""
    if i == 1:
        return tuple([True, True, True])
    elif i == 2:
        return tuple([True, True, False])
    elif i == 3:
        return tuple([True, False, True])
    elif i == 4:
        return tuple([True, False, True])
    elif i == 5:
        return tuple([True, False, False])



real_grid = [[0 for _ in range(5)] for _ in range(4)]


def print_grid(grid):
    """Prints a grid values"""
    for l in grid:
        for i in l:
            print(i, end='')
        print()
    print()


def debug_line_set():
    """Used to debug line sets"""
    for j in range(1, 6):
        # Set the line j with the previous value of the line j + 1
        set_line(real_grid, j, [x + 1 for x in get_line(real_grid, j)])

        # Print
        print('Line {} set:'.format(j))
        print_grid(real_grid)
