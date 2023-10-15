def problem_definition():
    return '''Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5
    spiral is formed as follows:

                                <21>  22   23   24  <25>
                                 20   <7>   8   <9>  10
                                 19    6   <1>   2   11
                                 18   <5>   4   <3>  12
                                <17>  16   15   14  <13>

    It can be verified that the sum of the numbers on the diagonals is 101.
    What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?'''


# index-0 based
def spiral_length(at):
    return 1 + 2 * at


# index-0 based
def spiral_border_size(at):
    # bit-shifting by 2 is the same as multiplying by 4
    # If we shifted in decimal, we'd be multiplying by 10
    # If we shifted in binary,  we'd be multiplying by 2
    if at == 0:
        return 1

    return (spiral_length(at) << 2) - 4


# index-0 based
def diagonal_sum(at, start_at=-1):
    if at == 0:
        return 1

    # If no previous value is provided, calculate it.
    # It's the sum of all the previous spiral sizes
    if start_at == -1:
        start_at = 0
        for x in range(at):
            start_at += spiral_border_size(x)

    # start_at is also the previous spiral size
    # (see top right corner of previous step)
    # c d e f g
    # b x x > 1
    # a x x x 2
    # 9 x x x 3
    # 8 7 6 5 4

    spilen_minus1 = spiral_length(at) - 1
    curpos = start_at
    result = 0
    # To get to the next corner, we need to add spiral_length - 1
    # Refer to the diagram above, we need to do it 4 times
    for x in range(4):
        curpos += spilen_minus1
        result += curpos

    return result


total = 0
start_at = 0
# Keep track of start_at locally to improve performance
for x in range(501):
    total += diagonal_sum(x, start_at)
    print('For a {0}x{0} spiral, total is {1}'.format(spiral_length(x), total))
    start_at += spiral_border_size(x)

print(total)
