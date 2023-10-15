def problem_definition():
    return '''The number, 197, is called a circular prime because all rotations of the digits:
    197, 971, and 719, are themselves prime.

    There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
    How many circular primes are there below one million?'''


def is_prime(n):
    if n == 2 or n == 3:
        return True
    if n % 2 == 0 or n < 2:
        return False
    for i in range(3, int(n**0.5) + 1, 2):  # only odd numbers
        if n % i == 0:
            return False
    return True


def cycles(n):
    nstr = str(n)
    for x in range(len(nstr)):
        yield int(nstr)
        # Cycle (first digit will become last)
        nstr = nstr[1:] + nstr[0:1]


def is_valid(n):
    for cycle in cycles(n):
        if not is_prime(cycle):
            return False
    return True


count = 0
for x in range(2, 1000000):
    if is_valid(x):
        count += 1
        print('{} -> found {} so far'.format(x, count))

print('There are {}'.format(count))
