from lw.primes import Primes


def problem_definition():
    return '''The first two consecutive numbers to have two distinct prime factors are:
                                     14 = 2 × 7
                                     15 = 3 × 5

    The first three consecutive numbers to have three distinct prime factors are:
                                    644 = 2² × 7 × 23
                                    645 = 3 × 5 × 43
                                    646 = 2 × 17 × 19.

    Find the first four consecutive integers to have four distinct prime factors.
    What is the first of these numbers?'''


def factors(n):
    primes = iter(Primes(100000))
    while n != 1:
        prime = next(primes)
        while n % prime == 0:
            n //= prime
            yield prime


def find(consecutive):
    print('Searching {} consecutive numbers...'.format(consecutive))
    for i in range(1, 1000000):
        ok = True
        for j in range(consecutive):
            if len(set(factors(i + j))) != consecutive:
                ok = False
                if j == 3:
                    print('{} is ok for 3 though'.format(i, consecutive))
                break

        if ok:
            print('{} is ok for {} numbers!'.format(i, consecutive))
            break


def find_any_consecutive(min_consecutive):
    print('Searching any consecutive numbers...')
    for i in range(1, 1000000):
        ifactors = len(set(factors(i)))
        if ifactors < min_consecutive:
            continue

        ok = True
        for j in range(1, ifactors):
            if len(set(factors(i + j))) != ifactors:
                ok = False
                break

        if ok:
            print('{} is ok for {} numbers!'.format(i, ifactors))


def find4consecutive():
    print('Searching 4 consecutive numbers...')
    for i in range(1, 1000000):
        if i % 1000 == 0:
            print('{}/1000000'.format(i))

        if len(set(factors(i+0))) != 4:
            continue
        if len(set(factors(i+1))) != 4:
            continue
        if len(set(factors(i+2))) != 4:
            continue
        if len(set(factors(i+3))) != 4:
            continue

        print('Found {}'.format(i))
        break

find4consecutive()
