import pickle
from os.path import isfile


def problem_definition():
    return '''A perfect number is a number for which the sum of its proper divisors is exactly equal to the
    number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means
    that 28 is a perfect number.

    A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant
    if this sum exceeds n.

    As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as
    the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater
    than 28123 can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced
    any further by analysis even though it is known that the greatest number that cannot be expressed as the
    sum of two abundant numbers is less than this limit.

    Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.'''


def numtype(number):
    """Determines the number type, deficient, perfect or abundant (-1, 0 or +1)"""
    sum_divisors = sum(div for div in range(1, number) if number % div == 0)
    if sum_divisors < number:
        return -1
    elif sum_divisors > number:
        return +1
    else:
        return +0  # it's perfect!

# Store all the abundant numbers
abundants = []
abundants_file = 'resources/p023_abundant_numbers.pickle'

# Check if we can load them from a previous session, else, generate them
if isfile(abundants_file):
    print('Abundant numbers file exists. Loading from file...', end=' ')
    with open(abundants_file, 'rb') as file:
        abundants = pickle.load(file)
    print('Done.')

else:
    print("Calculating abundant numbers...")
    abundants = [num for num in range(1, 28123) if numtype(num) == 1]
    print('Saving abundant numbers to file...', end=' ')
    with open(abundants_file, 'wb') as file:
        pickle.dump(abundants, file)
    print('Done.')

# This will store all the sums between abundant numbers
allsums = set()
# This will store all the numbers we need to check after
allnums = set([x for x in range(1, 28123)])

status = 0
maxstatus = len(abundants) ** 2
for a in abundants:
    for b in abundants:
        allsums.add(a + b)

        # Print status
        if status % 1000000 == 0:
            print("{:.2%}".format(status / maxstatus))
        status += 1

# The numbers which cannot be calculated as the sum of two abundant numbers are then:
print('Calculating set and sum...')
cannotbesum = allnums.difference(allsums)
print(sum(cannotbesum))
