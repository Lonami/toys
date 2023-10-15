# Class made by Lonami Exo (c) LonamiWebs
# Creation date: 13-07-2016


from lw.primes import Primes


def __get_until(n):
    """Determines until where we should calculate prime factors"""
    return int(n ** 0.5) + 1


def factors(n):
    """Yields the factors of n"""

    # Avoid the case where n = 1
    if n == 1:
        yield 1

    # If n is prime itself, we can early terminate
    elif Primes.is_prime(n):
        yield n

    # Otherwise, try to divide n by every prime and yield the factors
    else:
        primes = iter(Primes(__get_until(n)))

        # While we can divide it, do it
        while n != 1:
            try:
                # If we have a next prime and we can divide, do it
                prime = next(primes)
                while n % prime == 0:
                    n //= prime
                    yield prime
            except StopIteration:  # n is now prime itself
                yield n
                n = 1


def count_factors(n):
    """Counts the factors of n"""

    # If n equals 1 or is prime itself, it only has one factor
    # (since 1 is always a factor when n > 1, it is not counted)
    if n == 1 or Primes.is_prime(n):
        return 1
    else:
        count = 0
        primes = iter(Primes(__get_until(n)))

        # While we can find more factors
        while n != 1:
            try:
                # If we have a next prime and we can divide, do it
                prime = next(primes)
                while n % prime == 0:
                    n //= prime
                    count += 1

            except StopIteration:  # n is now prime itself
                count += 1
                n = 1

        return count


def count_divisors(n):
    """Counts the divisors of n

    How it works:
    Let N = p1^a1 * p2^a2 * ... * pn^an
    > Where pn is a prime and an its power

    The number of divisors can be calculated as follows:
    D(N) = (a1 + 1) * (a2 + 1) * ... * (an + 1)"""
    count = 1
    for _, power in factors_as_powers(n):
        count *= power + 1

    return count


def factors_as_powers(n):
    """Yields the factors of n grouped as (factor, power)"""

    # Avoid the case where n = 1
    if n == 1:
        yield (1, 1)

    # If n is prime itself, we can early terminate
    elif Primes.is_prime(n):
        yield (n, 1)
    else:
        primes = iter(Primes(__get_until(n)))
        while n != 1:
            try:
                # If we have a next prime and we can divide, do it
                prime = next(primes)
                power = 0
                while n % prime == 0:
                    power += 1
                    n //= prime

                # If we executed the while loop at least once, return this prime factor
                if power > 0:
                    yield (prime, power)

            except StopIteration:  # n is now prime itself
                yield (n, 1)
                n = 1


def lcm(numbers):
    """Determines the Least Common Multiple for a list of numbers"""
    # Keep track of the highest powers
    factor_powers = {}
    for n in numbers:
        for factor, power in factors_as_powers(n):
            if factor not in factor_powers or power > factor_powers[factor]:
                factor_powers[factor] = power

    # Then multiply them all together
    result = 1
    for factor, power in factor_powers.items():
        result *= factor ** power

    return result
