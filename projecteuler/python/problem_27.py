from math import sqrt


def problem_definition():
    return '''Euler discovered the remarkable quadratic formula:
                            n² + n + 41

    It turns out that the formula will produce 40 primes for the consecutive values n = 0 to 39.
    However, when n = 40, 402 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when
    n = 41, 41² + 41 + 41 is clearly divisible by 41.

    The incredible formula  n² − 79n + 1601 was discovered, which produces 80 primes for the consecutive
    values n = 0 to 79. The product of the coefficients, −79 and 1601, is −126479.

    Considering quadratics of the form:

        n² + an + b, where |a| < 1000 and |b| < 1000

        where |n| is the modulus/absolute value of n
        e.g. |11| = 11 and |−4| = 4

    Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum
    number of primes for consecutive values of n, starting with n = 0.'''


def is_prime(n):
    """Determines whether n is prime.
    Thanks to http://stackoverflow.com/a/18833870/4759433"""
    if n % 2 == 0 and n > 2:
        return False
    return all(n % i for i in range(3, int(sqrt(n)) + 1, 2))


def formula(a, b, n):
    return n**2 + a*n + b


def print_max(a, b, n):
    n -= 1
    if a == +1:
        print('New max found for n² + n + {0} for n = [0..{1}]'.format(b, n))
    elif a == -1:
        print('New max found for n² + -n + {1} for n = [0..{2}]'.format(b, n))
    else:
        print('New max found for n² + {0}n + {1} for n = [0..{2}]'.format(a, b, n))


max_a = 0
max_b = 0
max_n = 0
for a in range(-1000, 1000 + 1):
    for b in range(-1000, 1000 + 1):
        n = 0
        while is_prime(abs(formula(a, b, n))):
            n += 1

        if n > max_n:
            max_a = a
            max_b = b
            max_n = n
            print_max(a, b, n)

print('Result a*b = {}'.format(max_a * max_b))