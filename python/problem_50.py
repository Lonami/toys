from lw.primes import Primes


def problem_definition():
    return '''The prime 41, can be written as the sum of six consecutive primes:
                            41 = 2 + 3 + 5 + 7 + 11 + 13

    This is the longest sum of consecutive primes that adds to a prime below one-hundred.

    The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21 terms,
    and is equal to 953.

    Which prime, below one-million, can be written as the sum of the most consecutive primes?'''


done = False
maximum = 1000000
largestprime = 0
largestcount = 0

for _skip in range(maximum):
    if done:
        break

    firstprime = True
    primesum = 0
    countsum = 0
    skip = _skip
    for prime in Primes(maximum):
        # Skip the previously checked prime sum sequences
        if skip > 0:
            skip -= 1
            continue

        # If the first prime is already larger than the difference
        # between the latest largest sum and the maximum to check,
        # there is no point on carrying on checking, because any
        # sum will be larger with less numbers being summed
        if firstprime:
            firstprime = False
            if prime > maximum - largestprime:
                done = True
                break

        primesum += prime
        countsum += 1

        # We cannot check this, it is outside bounds
        if primesum >= maximum:
            break

        # We already found a larger one
        if countsum < largestcount:
            continue

        # Ensure it's prime, if it is, we found a new larger!
        if Primes.is_prime(primesum, False):
            Primes.ensure_cached_are_primes()
            largestprime = primesum
            largestcount = countsum

            print('Found {} which a sum of {} consecutive primes (from the {}th prime)'
                  .format(primesum, countsum, _skip + 1))
