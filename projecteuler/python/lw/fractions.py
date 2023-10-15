# Class made by Lonami Exo (c) LonamiWebs
# Creation date: 16-07-2016
# Edit 1       : 19-07-2016. Recursion -> Iteration, thanks to the forum


class PerfectSquareError(ValueError):
    """Occurs when a perfect square is given to a function that does not support it"""


# http://stackoverflow.com/a/15391420/4759433
# Using Newton's method
def isqrt(n):
    """Returns the integral square root of n by using the Newton's method"""
    x = n
    y = (2 ** ((n.bit_length() + 1) // 2)) - 1  # initial guess better than: (x + 1) // 2
    while y < x:
        x = y
        y = (x + n // x) // 2
    return x


# From https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Algorithm
def yield_continued_fraction_expansion(s):
    """Yields the items of the continued fraction for √s until it starts to repeat"""
    a0 = isqrt(s)
    if a0 * a0 == s:  # Perfect square
        raise PerfectSquareError("A perfect square was given")

    # Yield the first item
    yield a0

    # Initialize some variables
    m, d, a = 0, 1, a0

    # "The algorithm can also terminate on ai when ai = 2*a0"
    a0_double = a0 * 2
    while a != a0_double:
        m = d * a - m
        d = (s - m * m) // d
        a = (a0 + m) // d

        yield a


def yield_continued_fraction_expansion_forever(s):
    """Yields the continued fraction of √s forever.
    It first yields the first non-repeating item.
    Then, it yields forever the repeating items"""

    # Cache the list
    a = list(yield_continued_fraction_expansion(s))

    # Yield the first item and pop it out, since it doesn't repeat
    yield a.pop(0)

    # Yield items forever
    while True:
        for an in a:
            yield an


# From https://projecteuler.net/thread=66 by pesa
# Links (some are outdated):
# - http://en.wikipedia.org/wiki/Pell%27s_equation#Solution_technique
# ---> https://en.wikipedia.org/wiki/Pell%27s_equation#Solutions
# - http://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Continued_fraction_expansion
# - http://en.wikipedia.org/wiki/Simple_continued_fraction#Infinite_continued_fractions
def yield_continued_expansion(s):
    """Yields the (numerator, denominator) of the continued fraction of √s forever.
    Each iteration is more precise than the last one"""

    n, ln = 1, 0  # start with current numerator 1
    d, ld = 0, 1  # start with last denominator 1
    for a in yield_continued_fraction_expansion_forever(s):
        n, ln = (a * n) + ln, n  # to get the next numerator, multiply by the cycle and add the last
        d, ld = (a * d) + ld, d  # to get the next denominator, multiply by the cycle and add the last

        # This way, we'll get more and more precise values of √s given by n/d
        yield (n, d)


def sqrt_continued_fraction(n, precision=5):
    """Returns √n with the given precision, making use of a continue fraction"""

    # Get the generator and skip precision - 1 items
    cf = yield_continued_fraction_expansion(n)
    for i in range(1, precision):
        next(cf)

    # Get and return the last items
    n, d = next(cf)
    return n / d


# Old, using recursion
'''
def yield_continued_fraction_expansion_cycle(s):
    """Yields only the items of the repeating cycle in the continued fraction for √s"""
    a0 = isqrt(s)
    if a0 * a0 == s:  # Perfect square
        raise PerfectSquareError("A perfect square was given")

    # Initialize some variables
    m, d, a = 0, 1, a0

    # "The algorithm can also terminate on ai when ai = 2*a0"
    a0_double = a0 * 2
    while a != a0_double:
        m = d * a - m
        d = (s - m * m) // d
        a = (a0 + m) // d

        yield a


# Does this... really work though...? I mean, somehow, see problem_66.py
def get_cf(n, precision):
    """Gets the continued fraction of √n until the given precision"""

    # Calculate the first addition value
    add = isqrt(n)
    if precision == 0:
        return add

    # Get the continued fraction cycle for this square root
    cf_cycle = list(yield_continued_fraction_expansion_cycle(n))

    # Recurse
    return add + get_cf_omit_int_part(precision, cf_cycle, 0, len(cf_cycle))


def get_cf_omit_int_part(precision, cf_cycle, cf_index, cf_cycle_len):
    """Gets the continued fraction of √n until the given precision, omitting the integral part"""

    # Calculate how much we need to add now
    add = cf_cycle[cf_index % cf_cycle_len]

    # If precision is over, only return 1/add
    if precision == 1:
        return Fraction(1, add)

    # Else, recurse to the next level
    # Remember that each level is 1/(add + (1/add...))
    return Fraction(1, add + get_cf_omit_int_part(precision - 1, cf_cycle, cf_index, cf_cycle_len))


def get_cf_2(precision, add=1):
    """Gets the continued fraction of √2 until the given precision"""
    if precision == 1:
        return add + Fraction(1 / 2)
    else:
        return add + Fraction(1, 2 + get_cf_2(precision - 1, 0))


# See https://en.wikipedia.org/wiki/Continued_fraction#Motivation_and_notation
# e = [2;1,2,1,1,4,1,1,6,1,1,8,…] The pattern repeats indefinitely with a period of 3
#                                 except that 2 is added to one of the terms in each cycle.
#
# Step stands for the current step in the cyclic period
# It's used to determine whether 2 should be added
#
# Last add keeps track of the big add (2, 4, 6...)
#
# Note that precision is 0 based
def get_cf_e(precision, last_add=2, step=1):
    """Gets the continued fraction of e until the given precision"""

    # 0 times means nothing, no need to calculate
    if precision == 0:
        return Fraction(2, 1)

    # If the step is one on the period, use last_add
    if step % 3 == 0:
        add = last_add
        last_add += 2

    # Else if it's the first step, add is 2
    elif step == 1:
        add = 2

    # Else add is 1
    else:
        add = 1

    # Increment for the next step
    step += 1

    if precision == 1:
        # Calculate next add
        if step % 3 == 0:
            next_add = last_add
            # Don't increment last_add again since we're done
        else:
            next_add = 1

        return add + Fraction(1, next_add)

    else:
        # Recurse to the next level
        return add + Fraction(1, get_cf_e(precision - 1, last_add, step))
'''
