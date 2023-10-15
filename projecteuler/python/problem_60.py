from itertools import permutations

from lw.primes import Primes


def problem_definition():
    return '''The primes 3, 7, 109, and 673, are quite remarkable. By taking any two primes and concatenating
    them in any order the result will always be prime. For example, taking 7 and 109, both 7109 and 1097
    are prime. The sum of these four primes, 792, represents the lowest sum for a set of four primes with
    this property.

    Find the lowest sum for a set of five primes for which any two primes concatenate to produce another prime.'''


max_prime = 10000
print('Caching some primes...')
Primes(max_prime)
print('Doing work (please be patient)...')


# This general solution works for n
def check_primes(prime_count=5, primes=[], last_largest_prime=0):
    if len(primes) >= 2:  # perform checks as soon as we can for early termination
        # Concatenate all possible permutations and ensure they're primes
        for p in permutations(primes, 2):
            if not Primes.is_prime(int(str(p[0]) + str(p[1]))):
                return

        # We can avoid some combinations... (previously checked ones)
        if len(primes) == prime_count:
            print('Succeed for {}, which adds to {}'.format(primes, sum(primes)))

    if len(primes) < prime_count:  # carry on getting more primes
        for prime in Primes(max_prime):
            if prime <= last_largest_prime:
                continue  # do not use a lower prime (already used)

            # Clone list, append prime, and dive into the next recursion level
            next_primes = primes[:]
            next_primes.append(prime)
            check_primes(prime_count, next_primes, prime)


# However this, which doesn't use recursion, is faster
def specific_solution():
    for prime1 in Primes(max_prime):
        s_prime1 = str(prime1)

        for prime2 in Primes(max_prime):
            # Avoid checking the same primes
            if prime2 <= prime1:
                continue
            s_prime2 = str(prime2)

            # Check if concatenating is prime
            if not Primes.is_prime(int(s_prime1 + s_prime2)):
                continue
            if not Primes.is_prime(int(s_prime2 + s_prime1)):
                continue

            # print('Checking level 2: {}, {}'.format(prime1, prime2))
            for prime3 in Primes(max_prime):
                # Avoid checking the same primes
                if prime3 <= prime2:
                    continue
                s_prime3 = str(prime3)

                # Check if concatenating is prime
                if not Primes.is_prime(int(s_prime1 + s_prime3)):
                    continue
                if not Primes.is_prime(int(s_prime3 + s_prime1)):
                    continue
                if not Primes.is_prime(int(s_prime2 + s_prime3)):
                    continue
                if not Primes.is_prime(int(s_prime3 + s_prime2)):
                    continue

                # print('Checking level 3: {}, {}, {}'.format(prime1, prime2, prime3))
                for prime4 in Primes(max_prime):
                    # Avoid checking the same primes
                    if prime4 <= prime3:
                        continue
                    s_prime4 = str(prime4)

                    # Check if concatenating is prime
                    if not Primes.is_prime(int(s_prime1 + s_prime4)):
                        continue
                    if not Primes.is_prime(int(s_prime4 + s_prime1)):
                        continue
                    if not Primes.is_prime(int(s_prime2 + s_prime4)):
                        continue
                    if not Primes.is_prime(int(s_prime4 + s_prime2)):
                        continue
                    if not Primes.is_prime(int(s_prime3 + s_prime4)):
                        continue
                    if not Primes.is_prime(int(s_prime4 + s_prime3)):
                        continue

                    # print('Checking level 4: {}, {}, {}, {}'.format(prime1, prime2, prime3, prime4))
                    for prime5 in Primes(max_prime):
                        # Avoid checking the same primes
                        if prime5 <= prime4:
                            continue
                        s_prime5 = str(prime5)

                        # Check if concatenating is prime
                        if not Primes.is_prime(int(s_prime1 + s_prime5)):
                            continue
                        if not Primes.is_prime(int(s_prime5 + s_prime1)):
                            continue
                        if not Primes.is_prime(int(s_prime2 + s_prime5)):
                            continue
                        if not Primes.is_prime(int(s_prime5 + s_prime2)):
                            continue
                        if not Primes.is_prime(int(s_prime3 + s_prime5)):
                            continue
                        if not Primes.is_prime(int(s_prime5 + s_prime3)):
                            continue
                        if not Primes.is_prime(int(s_prime4 + s_prime5)):
                            continue
                        if not Primes.is_prime(int(s_prime5 + s_prime4)):
                            continue

                        print('We did it! {} + {} + {} + {} + {} = {}'
                              .format(prime1, prime2, prime3, prime4, prime5,
                                      prime1 + prime2 + prime3 + prime4 + prime5))

                        return


# check_primes(5)  # general solution
specific_solution()
