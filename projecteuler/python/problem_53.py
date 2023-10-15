from math import factorial


def problem_definition():
    return '''There are exactly ten ways of selecting three from five, 12345:
                123, 124, 125, 134, 135, 145, 234, 235, 245, and 345

    In combinatorics, we use the notation, ^5C_3 = 10.
    In general:
                              (n!)
                    ^nC_r = --------
                            r!(n−r)!

    Where r ≤ n, n! = n×(n−1)×...×3×2×1, and 0! = 1.

    It is not until n = 23, that a value exceeds one-million: ^23C_10 = 1144066.

    How many, not necessarily distinct, values of  ^nC_r, for 1 ≤ n ≤ 100, are greater than one-million?'''

# Note 1: I cannot write exponent-like characters, so I used ^c to indicate
# Note 2: Neither can write subscripts, so used _c to indicate


print(sum(1 for n in range(1, 100 + 1) for r in range(1, n + 1) if (factorial(n) // (factorial(r) * factorial(n - r))) > 1000000))

# Verbose
'''
def combinations(n, r):
    return factorial(n) // (factorial(r) * factorial(n - r))


count = 0
for n in range(1, 100 + 1):
    for r in range(1, n + 1):
        if combinations(n, r) > 1000000:
            count += 1

print(count)
'''