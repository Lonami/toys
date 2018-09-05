from itertools import permutations


def problem_definition():
    return '''We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n
    exactly once. For example, 2143 is a 4-digit pandigital and is also prime.

    What is the largest n-digit pandigital prime that exists?'''


# Courtesy of http://stackoverflow.com/a/15191619/4759433
# Not used though...
def is_pandigital9(n):
    """Checks whether a 9-digit-long number is pandigital.
    It will return False for any number with less than 9 digits"""
    if n != 9 * ((0x1c71c71d * n) >> 32):
        return False

    flags = 0
    while n > 0:
        q = (0x1999999a * n) >> 32
        flags |= 1 << (n - q * 10)
        n = q

    return flags == 0x3fe


# Courtesy of problem_37.py
def is_prime(n):
    if n == 2 or n == 3:
        return True
    if n % 2 == 0 or n < 2:
        return False
    for i in range(3, int(n**0.5) + 1, 2):  # only odd numbers
        if n % i == 0:
            return False
    return True


digits = '987654321'
while len(digits) > 0:
    for p in permutations(digits):
        n = int(''.join(p))
        if is_prime(n):
            print('Found the largest pandigital AND prime: {}'.format(n))
            digits = '0'
            break

    # Truncate left digit
    digits = digits[1:]
