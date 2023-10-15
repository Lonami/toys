from lw.primes import Primes


def problem_definition():
    return '''The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
    Find the sum of all the primes below two million.'''

print(sum(Primes(2000000)))
