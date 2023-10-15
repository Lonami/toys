def problem_definition():
    return '''A palindromic number reads the same both ways. The largest palindrome made from
    the product of two 2-digit numbers is 9009 = 91 × 99.

    Find the largest palindrome made from the product of two 3-digit numbers.'''


print(max(i * j for i in range(100, 1000) for j in range(i + 1, 1000) if str(i * j) == str(i * j)[::-1]))

# Verbose
'''
maximum = 0
for i in range(999, 99, -1):
    for j in range(i - 1, 99, -1):
        product = i * j
        products = str(product)
        # If the product string is equals to itself reversed, we did it!
        if product > maximum and products == products[::-1]:
            maximum = product
            print('{} with {} x {}'.format(product, i, j))
'''