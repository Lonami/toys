from itertools import permutations

from lw.primes import Primes


def problem_definition():
    return '''The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330,
    is unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit numbers
    are permutations of one another.

    There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this property,
    but there is one other 4-digit increasing sequence.

    What 12-digit number do you form by concatenating the three terms in this sequence?'''


def are_permutation(a, b):
    a = tuple(str(a))
    b = tuple(str(b))
    for permutation in permutations(a):
        if permutation == b:
            return True
    return False


for i in range(1000, 10000):
    if not Primes.is_prime(i):
        continue

    for j in range(1000, 10000):
        next2 = i + j + j
        if next2 > 9999:  # must be 4-digit
            break
        if not Primes.is_prime(next2):
            continue

        next1 = i + j
        if not Primes.is_prime(next1):
            continue

        if not are_permutation(i, next2):
            continue

        if not are_permutation(i, next1):
            continue

        print('Found sequence {}, {}, {} by adding {}'.format(i, next1, next2, j))
