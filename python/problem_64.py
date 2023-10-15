from lw.fractions import yield_continued_fraction_expansion_cycle


def problem_definition():
    return \
'''
All square roots are periodic when written as continued fractions and can be written in the form:

√N = a0 +        1
          -----------------
          a1 +      1
               -------------
               a2 +    1
                    --------
                    a3 + ...

For example, let us consider √23:

√23 = 4 + √23 - 4 = 4 +    1    = 4 +      1
                        -------       -----------
                           1              √23 - 3
                        -------       1 + -------
                        √23 - 4              7


If we continue we could get the following expansion:
√23 = 4 +          1
          -------------------
          1 +        1
              ---------------
              3 +      1
                  -----------
                  1 +    1
                      -------
                      8 + ...


The process can be summarised as follows:

           1      √23 + 4          √23 - 3
a0 = 4, ------- = -------    = 1 + -------
        √23 - 4      7                7


           7      7(√23 + 4)       √23 - 3
a1 = 1, ------- = ---------- = 3 + -------
        √23 - 3       14              2


           2      2(√23 + 4)       √23 - 4
a2 = 3, ------- = ---------- = 1 + -------
        √23 - 3       14              7


           7      7(√23 + 4)
a3 = 1, ------- = ---------- = 8 + √23 - 4
        √23 - 4      7


           1      √23 + 4          √23 - 3
a4 = 8, ------- = -------    = 1 + -------
        √23 - 4      7                7


           7      7(√23 + 4)       √23 - 3
a5 = 1, ------- = ---------- = 3 + -------
        √23 - 3       14              2


           2      2(√23 + 4)       √23 - 4
a6 = 3, ------- = ---------- = 1 + -------
        √23 - 3       14              7


           7      7(√23 + 4)
a7 = 1, ------- = ---------- = 8 + √23 - 3
        √23 - 4        7


It can be seen that the sequence is repeating.

For conciseness, we use the notation √23 = [4;(1,3,1,8)],
to indicate that the block (1,3,1,8) repeats indefinitely.

The first ten continued fraction representations of (irrational) square roots are:

√2=[1;(2)], period=1
√3=[1;(1,2)], period=2
√5=[2;(4)], period=1
√6=[2;(2,4)], period=2
√7=[2;(1,1,1,4)], period=4
√8=[2;(1,4)], period=2
√10=[3;(6)], period=1
√11=[3;(3,6)], period=2
√12= [3;(2,6)], period=2
√13=[3;(1,1,1,1,6)], period=5

Exactly four continued fractions, for N ≤ 13, have an odd period.

How many continued fractions for N ≤ 10000 have an odd period?
'''

#     Sum those in the given range which sum of items in the c.f. cycle          is odd
print(sum(1 for i in range(2, 10000 + 1) if sum(1 for j in yield_continued_fraction_expansion_cycle(i)) % 2 != 0))

# Broken somewhere by Lonami. Probably precision.
'''
def is_perfect_square(n):
    """Please pass a decimal!"""

    # If the last digit is 2, 3, 7 or 8, it can't be a perfect square
    # See http://burningmath.blogspot.com.es/2013/09/how-to-check-if-number-is-perfect-square.html
    last_digit = n % 10
    if (last_digit == 2 or
        last_digit == 3 or
        last_digit == 7 or
            last_digit == 8):
        return False

    # Otherwise, we can't do it anyhow else!
    return n == int(n.sqrt()) ** 2


# If we have the square root already, we can avoid some (expensive) calculations
def is_perfect_square_known_sqrt(n, n_sqrt):
    # If the last digit is 2, 3, 7 or 8, it can't be a perfect square
    # See http://burningmath.blogspot.com.es/2013/09/how-to-check-if-number-is-perfect-square.html
    last_digit = n % 10
    if (last_digit == 2 or
        last_digit == 3 or
        last_digit == 7 or
            last_digit == 8):
        return False

    # Otherwise, we can't do it anyhow else!
    return n == n_sqrt.to_integral(ROUND_FLOOR) ** 2


# http://math.stackexchange.com/a/716976
# x0 is our original number
# a0 is our number with the decimal part truncated
# b0 = x0 - a0 is our number's decimal part
def yield_an(x0, precision=750):
    """Please provide a decimal!"""
    a0 = x0.to_integral(ROUND_FLOOR)
    b0 = x0 - a0
    yield a0

    bn = b0
    while precision > 0:
        xn = 1 / bn
        an = xn.to_integral(ROUND_FLOOR)
        yield an
        bn = xn - an

        precision -= 1


# Old method used:
# subgroup = pattern[start_at:start_at + subgroup_size]
# next_subgroup = pattern[i:i + subgroup_size]
# if len(next_subgroup) < subgroup_size: break
# if subgroup != next_subgroup: ok = False
def find_pattern(pattern, start_at=1):
    """Pass a list please!"""

    # This works as follows.
    #
    # For example: [1, 1, 2, 1, 1, 2, 1, 1, 2...]
    # It will start checking the group [1]
    # But it will eventually reach the [2], this group didn't work!
    #
    # It will then use [1, 1]
    # But it will find [2, 1], so this group didn't work!
    #
    # It will then use [1, 1, 2]
    # And then it will find [1, 1, 2], and over and over.
    # This group did work

    # Store this variable to avoid calling len(...)
    pattern_size = len(pattern)

    # Start with a small group and ensure it repeats always
    for subgroup_size in range(1, pattern_size - start_at):

        ok = True  # Assume it's OK at the beginning
        # Avoid checking the first used group, check the rest only
        for i in range(start_at + subgroup_size, pattern_size, subgroup_size):

            # Check if we cannot check any longer
            # This may yield invalid results, which can be solved by using more precision
            if i + subgroup_size >= pattern_size:
                break

            # Do not slice the original array to get the current subgroups to check
            # Instead, check the group's elements one by one in both of them
            for j in range(subgroup_size):

                #  Original group           Next group to check
                if pattern[start_at + j] != pattern[i + j]:
                    ok = False
                    break

        # If this group was OK, return it! We're done
        if ok:
            return pattern[start_at:start_at + subgroup_size]

        # Else, we'll check with a larger group, until it fits the pattern exactly!
        # Or until we run out of precision

# 296 from [2..2000)
# Max period: 88 by √1726

# 1026 from [2000, 10000]
# Max period: 217 by √9949
odd_count = 0
print('Calculating...')

until = 10000
with localcontext() as ctx:
    ctx.prec = 1000  # Perform a high precision calculations

    for x in range(2, until + 1):
        x = Decimal(x)
        sqrt = x.sqrt()
        # if is_perfect_square(x):
        if is_perfect_square_known_sqrt(x, sqrt):
            continue

        a_n = list(yield_an(sqrt))
        pattern = find_pattern(a_n)
        precision_required = len(pattern)

        # printable_pattern = ','.join(str(i) for i in pattern)
        # print('√{}=[{};({})], period={}'.format(x, a_n[0], printable_pattern, precision_required))

        if precision_required % 2 != 0:  # odd
            odd_count += 1
            if odd_count % 10 == 0:
                print('[{:.2%}] New odd count: {}'.format(x / until, odd_count))

print('Final odd count: {}'.format(odd_count))
'''
