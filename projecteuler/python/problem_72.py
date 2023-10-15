#!/usr/bin/python3.6


def problem_definition():
    return '''Consider the fraction, n/d, where n and d are positive integers.
              If n<d and HCF(n,d)=1, it is called a reduced proper fraction.

              If we list the set of reduced proper fractions for d ≤ 8 in
              ascending order of size, we get:

                1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7,
                3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8

              It can be seen that there are 21 elements in this set.

              How many elements would be contained in the set of reduced proper
              fractions for d ≤ 1,000,000?'''


from fractions import Fraction
from lw.primeseq import infprimeseq





1/8
2/8 -> 1/4
3/8
4/8 -> 2/4 -> 1/4
5/8
6/8 -> 3/4
7/8

# If d is prime, it has d-1 proper fractions

d = 2
found = 1  # 1/2
found_set = set()
for p in infprimeseq():
    if p > 1000000:
        break
    found += p - 1
    # Let 'd' reach 'p'
    d += 1
    while d < p:
        d += 1
    
    # Get past 'p'
    d += 1
