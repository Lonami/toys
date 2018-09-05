def problem_definition():
    return '''We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n
    exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.

    The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier,
    and product is 1 through 9 pandigital.

    Find the sum of all products whose multiplicand/multiplier/product identity can be written as a
    1 through 9 pandigital.

    HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.'''


totalsum = 0
found = set()
digits = set(i for i in range(1, 10))


def check(multiplicand, multiplier):
    product = multiplicand * multiplier
    identity = str(multiplicand) + str(multiplier) + str(product)
    value = set(identity)

    if (len(identity) == 9 and
        len(value) == 9 and
        '0' not in value and
            product not in found):

        global totalsum
        totalsum += product
        found.add(product)
        print('Found another: {}*{} = {}, which adds to {}'.format(
            multiplicand, multiplier, product, totalsum))

for multiplicand in range(1, 10):
    for multiplier in range(1000, 10000):
        check(multiplicand, multiplier)

for multiplicand in range(10, 100):
    for multiplier in range(100, 1000):
        check(multiplicand, multiplier)

# THANKS ALBERT \(^.^)/ I UNDERSTOOD THE HINT WRONG, fixed now though!
