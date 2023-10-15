from lw.fractions import yield_continued_expansion, isqrt


def problem_definition():
    return '''Consider quadratic Diophantine equations of the form:

                                x² – Dy² = 1

    For example, when D=13, the minimal solution in x is 649² – 13×180² = 1.
    It can be assumed that there are no solutions in positive integers when D is square.

    By finding minimal solutions in x for D = {2, 3, 5, 6, 7}, we obtain the following:

    3² – 2×2² = 1
    2² – 3×1² = 1
    9² – 5×4² = 1
    5² – 6×2² = 1
    8² – 7×3² = 1

    Hence, by considering minimal solutions in x for D ≤ 7, the largest x is obtained when D=5.
    Find the value of D ≤ 1000 in minimal solutions of x for which the largest value of x is obtained.'''


def is_perfect_square(n):
    s = isqrt(n)
    return n == s * s


# Find the solution using continued fractions
max_x = 0
max_d = 0
for d in range(2, 1000 + 1):
    if is_perfect_square(d):
        continue

    # Get the next numerator/denominator pair
    for x, y in yield_continued_expansion(d):
        # Check if this pair satisfies the equation
        if x * x - d * y * y == 1:
            # It does! Store the new max values if greater
            if x > max_x:
                max_x = x
                max_d = d

            break

print(max_d)


# Too slow, too
'''
# When d = 109, x = 158070671986249
# So... this kind of stops working.
# See https://en.wikipedia.org/wiki/Pell%27s_equation#The_smallest_solution_of_Pell_equations
def find_diophantine_solution_solving_equation(d, maximum=10000000):
    # x² – Dy² = 1
    # –Dy² = 1 – x²
    # Dy² = x² – 1
    # y² = (x² – 1) / D
    # y = √((x² – 1) / D)

    x = d
    while x < maximum:
        dy_squared = (x * x - 1)
        x += 1

        y_squared = dy_squared // d

        # If the division was non-integer, this won't be equal
        if y_squared * d != dy_squared:
            continue

        y = Primes.isqrt(y_squared)
        # Integral square root was exact, this IS the solution!
        if y * y == y_squared:
            return x, y

    return -1, -1

# Since some cases are impossible, our safest bet is: if we can't find it, it must be really large!
# If we refer to https://en.wikipedia.org/wiki/Pell%27s_equation#Example we find:
#   "Values of n such that the smallest solution of x² – Dy² = 1
#    is greater than the smallest solution for any smaller value of n are:
#    1, 2, 5, 10, 13, 29, 46, 53, 61, 109, 181, 277, 397, 409, 421, 541, 661, 1021, 1069, 1381..."
# And since we need to look below 1001, 661 is our safest bet!
def main():
    largest_x = 0
    for i in range(2, 1000 + 1):
        if Primes.is_perfect_square(i):
            continue

        x, y = find_diophantine_solution_solving_equation(i)
        if x < 0:
            print('New suspicious. d = {}'.format(i))

        elif x > largest_x:
            largest_x = x
            print('New largest x. Solution = {0}² - {1}×{2}² = 1'.format(x, i, y))


main()
'''

# Too slow, even more than ^
'''
def find_diophantine_solution_brute_force(d):
    x = 0
    while True:
        x += 1

        # Reset both y and solution
        y = 0
        solution = 2  # initial value larger than 1 to enter the loop
        while solution > 1:
            y += 1
            solution = x * x - d * y * y

        # Return (x, y) if we found a solution
        if solution == 1:
            return x, y
'''
