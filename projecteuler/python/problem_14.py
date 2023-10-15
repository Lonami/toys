from lw.collatz import Collatz


def problem_definition():
    return '''Longest Collatz sequences

    The following iterative sequence is defined for the set of positive integers:

                        n → n/2 (n is even)
                        n → 3n + 1 (n is odd)

    Using the rule above and starting with 13, we generate the following sequence:
    13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

    It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
    Although it has not been proved yet (Collatz Problem), it is thought that all starting
    numbers finish at 1.

    Which starting number, under one million, produces the longest chain?

    NOTE: Once the chain starts the terms are allowed to go above one million.'''


# Test only odd numbers which have higher chance
# Also, test only those above half a million (others highly have less chance)
maximum = 0
for i in range(500001, 1000000, 2):
    collatz_length = sum(1 for _ in Collatz(i))
    if collatz_length > maximum:
        maximum = collatz_length
        print('New maximum length of {} for {}'.format(maximum, i))
