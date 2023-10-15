from itertools import permutations

from lw.primes import Primes


def problem_definition():
    return '''By replacing the 1st digit of the 2-digit number *3, it turns out that
    six of the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.

    By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit
    number is the first example having seven primes among the ten generated numbers,
    yielding the family: 56003, 56113, 56333, 56443, 56663, 56773, and 56993.
    Consequently 56003, being the first member of this family, is the smallest prime with this property.

    Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits)
    with the same digit, is part of an eight prime value family.'''


def families(n):
    digits = str(n)
    digitcount = len(digits)

    for i in range(0, digitcount - 1):
        # Step 1. Create n tuples which contains how many digits will be replaced
        digits_to_replace = tuple(j <= i for j in range(digitcount))

        # Step 2. Iterate over the unique permutations
        for replace_permutation in set(permutations(digits_to_replace)):
            # If the first digit will change,
            # we cannot start at 0 or it would have less digits!
            if replace_permutation[0]:
                start_at = 1
            else:
                start_at = 0

            def family():
                # Step 3. Change the digits where it should be replaced (it's True in the tuple)
                for j in range(start_at, 10):
                    newdigits = list(digits)
                    for k in range(digitcount):
                        if replace_permutation[k]:
                            newdigits[k] = str(j)

                    yield int(''.join(newdigits))

            yield family


def search():
    max_goal = 8
    max_primes = 0
    for prime in Primes(1000000):
        for fam in families(prime):
            prime_count = 0
            for child in fam():
                if Primes.is_prime(child):
                    prime_count += 1

            if prime_count > max_primes:
                max_primes = prime_count
                print('New max of {} for prime {}'.format(prime_count, prime))
                for child in fam():
                    if Primes.is_prime(child):
                        print(child, end=' ')
                print()
                if prime_count == max_goal:
                    return

search()
# Note that the solution if the first permutation, not the prime itself

'''
# OK, this works, it yields all the families and permutations
def check(n):
    digits = str(n)
    digitcount = len(digits)
    digitcountminus1 = digitcount - 1

    for i in range(0, digitcountminus1):
        # Step 1. Create n tuples which contains how many digits will be replaced
        digits_to_replace = tuple(j <= i for j in range(digitcount))

        # Step 2. Iterate over the unique permutations
        for replace_permutation in set(permutations(digits_to_replace)):

            # Step 3. Change the digits where it should be replaced (it's True in the tuple)
            for j in range(10):
                newdigits = list(digits)
                for k in range(digitcount):
                    if replace_permutation[k]:
                        newdigits[k] = str(j)

                newdigits = int(''.join(newdigits))
                yield newdigits'''
