#!/usr/bin/python3.6


def problem_definition():
    return '''The number 145 is well known for the property that the sum of the
              factorial of its digits is equal to 145:

                1! + 4! + 5! = 1 + 24 + 120 = 145

              Perhaps less well known is 169, in that it produces the longest
              chain of numbers that link back to 169; it turns out that there
              are only three such loops that exist:

                169 → 363601 → 1454 → 169
                871 → 45361 → 871
                872 → 45362 → 872

              It is not difficult to prove that EVERY starting number will
              eventually get stuck in a loop. For example,

                69 → 363600 → 1454 → 169 → 363601 (→ 1454)
                78 → 45360 → 871 → 45361 (→ 871)
                540 → 145 (→ 145)

              Starting with 69 produces a chain of five non-repeating terms,
              but the longest non-repeating chain with a starting number below
              one million is sixty terms.

              How many chains, with a starting number below one million,
              contain exactly sixty non-repeating terms?'''


from math import factorial

def nxt(n):
    return sum(int(factorial(int(d))) for d in str(n))


def chain(n):
    found = {n}
    n = nxt(n)
    while n not in found:
        found.add(n)
        n = nxt(n)
    return found


count = 0
for i in range(1, 1000000):
    if len(chain(i)) == 60:
        count += 1
        print(f'[{i/1000000:.2%}] {count}')

