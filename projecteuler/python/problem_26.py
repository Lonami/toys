def problem_definition():
    return '''A unit fraction contains 1 in the numerator. The decimal representation of the unit
    fractions with denominators 2 to 10 are given:

    1/2	 = 0.5
    1/3	 = 0.(3)
    1/4	 = 0.25
    1/5	 = 0.2
    1/6	 = 0.1(6)
    1/7	 = 0.(142857)
    1/8	 = 0.125
    1/9	 = 0.(1)
    1/10 = 0.1

    Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit
    recurring cycle.

    Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.'''


def to_decimal_string(numerator, denominator):
    """Converts a fraction to a decimal representation.
    A cycle is represented as (cycle). For example:
    1/3 = 0.333333... = 0.(3)

    Original source from http://stackoverflow.com/a/36560609/4759433"""

    # Start the result string before the decimal point
    result = [str(numerator // denominator) + '.']

    # Keep track of the found remainders
    subresults = [numerator % denominator]

    # The new numerator is now the previous reminder
    numerator %= denominator

    # While the numerator is not 0, we can carry on dividing
    while numerator != 0:
        # Multiply it by 10 to get more precision
        numerator *= 10

        # result_digit is now the floor division between numerator and denominator
        # numerator is now the reminder of this division
        result_digit, numerator = divmod(numerator, denominator)

        # Append the next floor division result to the result
        result.append(str(result_digit))

        # If the new remainder isn't yet in the sub-results, append it
        if numerator not in subresults:
            subresults.append(numerator)

        # Otherwise, it means that we already found that remainder!
        else:
            # Find the index of this remainder, and add the parenthesis
            # that indicate a cyclic block
            result.insert(subresults.index(numerator) + 1, "(")
            result.append(")")
            break

    return "".join(result).rstrip('.')


def count_cyclic_length(numerator, denominator):
    result = [str(numerator // denominator) + '.']
    subresults = [numerator % denominator]
    numerator %= denominator

    while numerator != 0:
        numerator *= 10
        result_digit, numerator = divmod(numerator, denominator)
        result.append(str(result_digit))

        if numerator not in subresults:
            subresults.append(numerator)
        else:
            return len(subresults) - subresults.index(numerator)

    return 0


max_cyclic = 0
for x in range(1, 1000):
    cyclic = count_cyclic_length(1, x)
    if cyclic > max_cyclic:
        print('1/{} has cyclic of {}'.format(x, count_cyclic_length(1, x)))
        max_cyclic = cyclic
