# Class made by Lonami Exo (c) LonamiWebs
# Suggestions from stackoverflow.com/a/18833870/4759433
#
# Creation date: 10-07-2016
# Edit 1       : 12-07-2016: Improved cache system
# Edit 2       : 13-07-2016: Added documentation, index to access n'th prime,
#                            a function to cache the next n primes and
#                            implemented "contains" for using as "n in primes"
# Edit 3:      : 16-07-2016: Added isqrt method and is perfect square


class Primes:
    """Iterator that yields prime numbers using a cache system.
    To initialize the static cache, call the constructor with a maximum prime"""
    __cached_primes = [2, 3]  # start with some (required for some methods)
    __cached_until = 3
    
    def __init__(self, maximum=0):
        self.max = maximum
        if maximum > Primes.__cached_until:
            Primes.__build_cache(maximum)

    def __iter__(self):
        self.index = -1
        return self

    def __next__(self):
        # Increase index for the next time and return prime
        self.index += 1

        # If we can't use the next index, it means the next prime is over the max
        if self.index >= len(Primes.__cached_primes):
            raise StopIteration

        # If the cached prime is over the maximum, the next prime is also over the max
        if Primes.__cached_primes[self.index] > self.max:
            raise StopIteration

        return Primes.__cached_primes[self.index]

    def __getitem__(self, index):
        return Primes.prime_at(index)

    def __contains__(self, n):
        return Primes.is_prime(Primes.is_prime(n))

    # http://stackoverflow.com/a/15391420/4759433
    # Using Newton's method
    @staticmethod
    def isqrt(n):
        x = n
        y = (2 ** ((n.bit_length() + 1) // 2)) - 1  # initial guess better than: (x + 1) // 2
        while y < x:
            x = y
            y = (x + n // x) // 2
        return x

    @staticmethod
    def is_perfect_square(n):
        sqrt = Primes.isqrt(n)
        return n == sqrt * sqrt

    @staticmethod
    def prime_at(index):
        # If the index is outside the cached bounds, find it first
        if index >= len(Primes.__cached_primes):
            Primes.__cache_next(1 + index - len(Primes.__cached_primes))

        # Return the prime
        return Primes.__cached_primes[index]

    @staticmethod
    def is_prime(n):  # Always use cache
        """Determines whether a positive integer n is prime or not.

        Note that this method does NOT check if the integer is greater than 0
        for performance reasons.

        Also do note that this will build more cache if necessary for later use."""
        until = Primes.isqrt(n) + 1
        if n > Primes.__cached_until:
            Primes.__build_cache(until)

        # If n can't be divided by any prime below it's square root,
        # we can conclude that n is prime, because we've built enough cache!
        for prime in Primes.__cached_primes:
            if n % prime == 0:
                return False

            if prime > until:
                return True

        return True

    @staticmethod
    def __is_prime(n):
        """Determines whether a number is prime, by first checking the cache"""
        # Default values (avoid checking all the all-checked primes)
        start_from = Primes.__cached_primes[-1] + 2
        until = int(n**0.5) + 1

        # Instead dividing for every number, first try primes only (others are multiples)
        for prime in Primes.__cached_primes:
            if prime > until:
                break  # Stop checking, it may not be cached yet

            if n % prime == 0:
                return False

        # If there was no luck yet, we need to check this way
        for i in range(start_from, until, 2):  # only odd numbers
            if n % i == 0:
                return False

        # It looks like there was luck, we couldn't divide this number!
        return True

    @staticmethod
    def __build_cache(until):
        """Build cache until a given prime"""
        # Start from the last cached prime + 2 (so we don't add it twice)
        start_from = Primes.__cached_primes[-1] + 2

        # Until + 1 because range compares with <, not <=
        for i in range(start_from, until + 1, 2):
            if Primes.__is_prime(i):
                Primes.__cached_primes.append(i)

        Primes.__cached_until = until

    @staticmethod
    def __cache_next(n):
        """Cache the n next primes"""
        # Start from the last cached prime + 2 (so we don't add it twice)
        i = Primes.__cached_primes[-1]

        # While we must cache, carry on doing so
        while n > 0:
            i += 2
            if Primes.__is_prime(i):
                Primes.__cached_primes.append(i)
                Primes.__cached_until = i
                n -= 1
