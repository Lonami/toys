def problem_definition():
    return '''-- Champernowne's constant --

    An irrational decimal fraction is created by concatenating the positive integers:

                        0.123456789101112131415161718192021...

    It can be seen that the 12th digit of the fractional part is 1.
    If dn represents the nth digit of the fractional part, find the value of the following expression.
    d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000'''


# Non-optimal solution, but it works
fpart = '0'  # start as 0 to omit the 0-based part
for i in range(1, 1000000):
    fpart += str(i)

for i in range(0, 7):
    print(fpart[10 ** i], end=' x ')

print('-> ')
print(int(fpart[1]) *
      int(fpart[10]) *
      int(fpart[100]) *
      int(fpart[1000]) *
      int(fpart[10000]) *
      int(fpart[100000]) *
      int(fpart[1000000]))
