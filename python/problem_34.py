from math import factorial


def problem_definition():
    return '''145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
    Find the sum of all numbers which are equal to the sum of the factorial of their digits.
    Note: as 1! = 1 and 2! = 2 are not sums they are not included.'''

print(sum(x for x in range(3, 50000) if sum(factorial(int(c)) for c in str(x)) == x))

# Or verbose way:
'''def is_valid(number):
    sum = 0
    for c in str(number):
        sum += factorial(int(c))
        if sum > number:
            return False
    return sum == number


sum = 0
for x in range(3, 50000):
    if is_valid(x):
        print(x)
        sum += x

print('Result: {}'.format(sum))'''
