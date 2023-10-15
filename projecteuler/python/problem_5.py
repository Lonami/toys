from lw.factors import lcm


def problem_definition():
    return '''2520 is the smallest number that can be divided by each of the numbers from
    1 to 10 without any remainder.

    What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?'''


print(lcm(i for i in range(1, 20 + 1)))
