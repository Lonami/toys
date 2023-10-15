def problem_definition():
    return '''A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
                                    a² + b² = c²

    For example, 3² + 4² = 9 + 16 = 25 = 5².

    There exists exactly one Pythagorean triplet for which a + b + c = 1000.
    Find the product abc.'''


for a in range(1, 1000):
    for b in range(a + 1, 1000 - a):
        c = 1000 - a - b
        if a ** 2 + b ** 2 == c ** 2:
            print('Found a = {}, b = {}, c = {}, a*b*c = {}'
                  .format(a, b, c, a * b * c))
