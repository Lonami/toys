from math import gcd
from time import sleep


def problem_definition():
    return '''Euler's Totient function, φ(n) [sometimes called the phi function],
    is used to determine the number of numbers less than n which are relatively prime to n.
    For example, as 1, 2, 4, 5, 7, and 8, are all less than nine and relatively prime to nine, φ(9)=6.

    ┌────┬──────────────────┬──────┬──────────┐
    │ n  │ Relatively Prime │ φ(n) │  n/φ(n)  │
    ├────┼──────────────────┼──────┼──────────┤
    │ 2  │ 1                │  1   │ 2        │
    │ 3  │ 1,2              │  2   │ 1.5      │
    │ 4  │ 1,3              │  2   │ 2        │
    │ 5  │ 1,2,3,4          │  4   │ 1.25     │
    │ 6  │ 1,5              │  2   │ 3        │
    │ 7  │ 1,2,3,4,5,6      │  6   │ 1.1666...│
    │ 8  │ 1,3,5,7          │  4   │ 2        │
    │ 9  │ 1,2,4,5,7,8      │  6   │ 1.5      │
    │ 10 │ 1,3,7,9          │  4   │ 2.5      │
    └────┴──────────────────┴──────┴──────────┘

    It can be seen that n=6 produces a maximum n/φ(n) for n ≤ 10.
    Find the value of n ≤ 1,000,000 for which n/φ(n) is a maximum.'''

def φ(n):
    result = 0
    for i in range(1, n):
        # If the only number that can evenly divide both i and n is 1,
        # then it is co-prime. This is the same as saying that their gcd equals 1
        if gcd(i, n - i) == 1:
            result += 1

    return result


def main():
    maximum = 0
    # Checked up to 40,000
    # New maximum for n = 30030, φ(n) = 5760
    for n in range(2, 1000000 + 1):
        if n % 1000 == 0:
            print("At {}".format(n))

        numbers = φ(n)
        if n / numbers > maximum:
            maximum = n / numbers
            print('New maximum for n = {}, φ(n) = {}'.format(n, numbers))

main()


def test(until):
    for n in range(2, until):
        print('n = {}, φ(n) = {}'.format(n, φ(n)))


# https://en.wikipedia.org/wiki/Coprime_integers#Generating_all_coprime_pairs
def generate_tree1(until):
    # Since m is multiplied by 2 and we don't want members
    # larger than until the one we want
    until //= 2

    # Start at (2, 1) for even-odd and odd-even pairs
    m, n = 2, 1
    while m < until:
        yield (2 * m - n, m)  # Branch 1
        yield (2 * m + n, m)  # Branch 2
        yield (m + 2 * n, n)  # Branch 3

        m += 1
        n += 1


def generate_tree2(until):
    # Since m is multiplied by 2 and we don't want members
    # larger than until the one we want
    until //= 2

    # Start at (3, 1) for odd-odd pairs
    m, n = 3, 1
    while m < until:
        yield (2 * m - n, m)  # Branch 1
        yield (2 * m + n, m)  # Branch 2
        yield (m + 2 * n, n)  # Branch 3

        m += 1
        n += 1


stored = {}
for a, b in generate_tree1(10000):
    if a in stored:
        stored[a] += 1
    else:
        stored[a] = 1
    if b in stored:
        stored[b] += 1
    else:
        stored[b] = 1

for a, b in generate_tree2(10000):
    if a in stored:
        stored[a] += 1
    else:
        stored[a] = 1
    if b in stored:
        stored[b] += 1
    else:
        stored[b] = 1

print(stored)
