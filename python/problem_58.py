# Borrowed from problem_28.py
# Although the spiral is made in the wrong order,
# we can just skip the last item in the diagonal,
# and it will work the same
from lw.primes import Primes


def problem_definition():
    return '''Starting with 1 and spiralling anticlockwise in the following way,
    a square spiral with side length 7 is formed.

                           <37>  36   35   34   33   32  <31>
                            38  <17>  16   15   14  <13>  30
                            39   18   <5>   4   <3>  12   29
                            40   19    6    1    2   11   28
                            41   20   <7>   8    9   10   27
                            42  <21>  22   23   24   25   26
                           <43>  44   45   46   47   48   49

    It is interesting to note that the odd squares lie along the bottom right diagonal,
    but what is more interesting is that 8 out of the 13 numbers lying along both diagonals
    are prime; that is, a ratio of 8/13 ≈ 62%.

    If one complete new layer is wrapped around the spiral above, a square spiral with side
    length 9 will be formed. If this process is continued, what is the side length of the
    square spiral for which the ratio of primes along both diagonals first falls below 10%?'''


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
def get_diagonals(at):
    if at == 0:
        yield 1
        return

    # We should start at the sum of all the previous spiral sizes
    curpos = sum(spiral_border_size(x) for x in range(at))

    # cur pos starts at the previous spiral size
    # (see top right corner of previous step)
    # c d e f g
    # b x x > 1
    # a x x x 2
    # 9 x x x 3
    # 8 7 6 5 4

    spilen_minus1 = spiral_length(at) - 1
    # To get to the next corner, we need to add spiral_length - 1
    # Refer to the diagram above, we need to do it 4 times
    for x in range(4):
        curpos += spilen_minus1
        yield curpos


def work():
    prime_count = 0
    number_count = 1  # skip 1, so we never get the ratio 0/1
    for x in range(1, 20000):
        for n in get_diagonals(x):
            number_count += 1
            if Primes.is_prime(n):
                prime_count += 1

        ratio = prime_count / number_count
        if x % 100 == 0:
            print('{:.2%}'.format(ratio))

        if ratio < 0.1:  # less than 10%
            print(spiral_length(x))
            return

work()
