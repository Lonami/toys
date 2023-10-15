def problem_definition():
    return '''Goldbach's other conjecture

    It was proposed by Christian Goldbach that every odd composite number can be written as the sum of
    a prime and twice a square.

        9  = 7  + 2×1²
        15 = 7  + 2×2²
        21 = 3  + 2×3²
        25 = 7  + 2×3²
        27 = 19 + 2×2²
        33 = 31 + 2×1²

    It turns out that the conjecture was false.

    What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?'''


# Note: composite = not prime
# Courtesy of problem_41.py, of problem_37.py
def is_prime(n):
    if n == 2 or n == 3:
        return True
    if n % 2 == 0 or n < 2:
        return False
    for i in range(3, int(n**0.5) + 1, 2):  # only odd numbers
        if n % i == 0:
            return False
    return True


print('Calculating some primes...')
primes = [prime for prime in range(2, 100000) if is_prime(prime)]


def is_ok(n):
    for prime in primes:
        if prime > n:
            return False

        for i in range(1, 1000):
            result = prime + 2 * i ** 2
            if result > n:
                break

            if result == n:
                return True


print('Doing actual work...')
for m in range(3, 100000, 2):
    if m not in primes and not is_ok(m):
        print('{} is not ok'.format(m))
        break
