from lw.factors import factors


def problem_definition():
    return '''The prime factors of 13195 are 5, 7, 13 and 29.
    What is the largest prime factor of the number 600851475143?'''


print(tuple(factors(600851475143))[-1])
