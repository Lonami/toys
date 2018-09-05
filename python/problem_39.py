def problem_definition():
    return '''If p is the perimeter of a right angle triangle with integral length sides, {a,b,c},
    there are exactly three solutions for p = 120.

    {20,48,52}, {24,45,51}, {30,40,50}

    For which value of p ≤ 1000, is the number of solutions maximised?'''


# Returns 0 if it doesn't have
def ihypotenuse(a, b):
    a *= a
    b *= b
    sumab = a + b
    isqrt = int(sumab ** 0.5)
    if isqrt * isqrt != sumab:  # It wasn't integral :(
        return 0

    return isqrt


perimeter_solutioncount = {}

for a in range(1, 10000):
    for b in range(1, 10000):
        # Avoid non-necessary tests (perimeter is already larger than 1000!)
        if a + b > 1000:
            continue

        # Calculate an integral hypotenuse
        c = ihypotenuse(a, b)

        # If it's non-zero, then it has an hypotenuse
        if c:
            # Calculate the perimeter
            perimeter = a + b + c
            if perimeter < 1000:
                # As it was inside the bounds, add a solution
                if perimeter not in perimeter_solutioncount:
                    perimeter_solutioncount[perimeter] = 1
                else:
                    perimeter_solutioncount[perimeter] += 1

    if a % 100 == 0:
        print('{0:.2%}'.format(a / 10000))


print('Searching the perimeter with the most solutions... ', end='')
max_p = 0
max_s = 0
for p, s in perimeter_solutioncount.items():
    if s > max_s:
        max_p = p
        max_s = s

print('{}, with {} solutions'.format(max_p, max_s))
