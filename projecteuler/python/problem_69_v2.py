from math import gcd
from time import sleep

from lw.primeseq import infprimeseq


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


primes = infprimeseq()


def get_factors(n):
    """Returns all the prime factors of n"""
    factors = []
    for p in primes:
        c, r = divmod(n, p)
        # While its divisible
        while r == 0:
            # Add this factor
            factors.append(p)
            # Update with the new value after dividing
            n = c
            # And calculate the next division
            c, r = divmod(n, p)
        
        # Once the number is 1, we're done dividing
        if n == 1:
            return factors


def euler_phi(n):
    """Calculates the Euler's Phi function, also known as the
       totient function, by using the prime factors of n"""
    result = n
    for p in set(get_factors(n)):
        result *= (1 - (1 / p))
    return int(result)


def main():
    maximum = 0
    # Checked up to 646,000
    # New maximum for n = 510510, φ(n) = 92160
    #for n in range(2, 1000000 + 1):
    #
    # At 646000 I stopped, now going on steps of 2
    # Before 646000, all steps of 1
    # Before 920000, all steps of 2
    maximum = 5.539388020833333
    for n in range(920000, 1000000 + 1, 5):
        if n % 1000 == 0:
            print("At {}".format(n))

        numbers = euler_phi(n)
        if n / numbers > maximum:
            maximum = n / numbers
            print('New maximum for n = {}, φ(n) = {}'.format(n, numbers))

main()

