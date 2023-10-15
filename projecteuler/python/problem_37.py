def problem_definition():
    return '''The number 3797 has an interesting property. Being prime itself, it is
    possible to continuously remove digits from left to right, and remain prime at each stage:
    3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.

    Find the sum of the only eleven primes that are both truncatable from left to right and right to left.

    NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.'''


def is_prime(n):
    if n == 2 or n == 3:
        return True
    if n % 2 == 0 or n < 2:
        return False
    for i in range(3, int(n**0.5) + 1, 2):  # only odd numbers
        if n % i == 0:
            return False
    return True


# Truncate from left to right
def truncate_ltr(n):
    nstr = str(n)
    for x in range(0, len(nstr)):
        yield int(nstr)
        # Truncate (first digit will be truncated)
        nstr = nstr[1:]


# Truncate from right to left
def truncate_rtl(n):
    nstr = str(n)
    for x in range(0, len(nstr)):
        yield int(nstr)
        # Truncate (first digit will be truncated)
        nstr = nstr[:-1]


# Test number from left to right
def passes_ltr(n):
    for t in truncate_ltr(n):
        if not is_prime(t):
            return False
    return True


# Test number from right to left
def passes_rtl(n):
    for t in truncate_rtl(n):
        if not is_prime(t):
            return False
    return True


found_sum = 0     # keep track of the found sum
left_primes = 11  # given by the problem
number = 9        # start from 9 (as noted by the problem)

while left_primes > 0:
    number += 1
    # Does this number pass both tests?
    if not passes_ltr(number):
        continue
    if not passes_rtl(number):
        continue

    # Test passed!
    left_primes -= 1
    found_sum += number
    print('Found {}, {} left!'.format(number, left_primes))

print('End sum is {}'.format(found_sum))
