def problem_definition():
    return '''The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
    Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
    (Please note that the palindromic number, in either base, may not include leading zeros.)'''

#     sum numbers    under one million if decimal is palindromic and             in binary too
print(sum(n for n in range(1, 1000000) if str(n) == str(n)[::-1] and "{0:b}".format(n) == "{0:b}".format(n)[::-1]))
