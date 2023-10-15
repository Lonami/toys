def problem_definition():
    return '''The Fibonacci sequence is defined by the recurrence relation:

        Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.

    Hence the first 12 terms will be:
        F1 = 1
        F2 = 1
        F3 = 2
        F4 = 3
        F5 = 5
        F6 = 8
        F7 = 13
        F8 = 21
        F9 = 34
        F10 = 55
        F11 = 89
        F12 = 144

    The 12th term, F12, is the first term to contain three digits.
    What is the index of the first term in the Fibonacci sequence to contain 1000 digits?'''


def fib():
    a, b = 1, 1
    while True:
        yield a
        a, b = b, a + b

index = 0
for f in fib():
    index += 1
    if len(str(f)) == 1000:  # first 1000 digits fibonacci number!
        print(f)
        print('at {}'.format(index))
        break
