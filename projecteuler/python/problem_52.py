def problem_definition():
    return '''It can be seen that the number, 125874, and its double, 251748, contain exactly
    the same digits, but in a different order.

    Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.'''


def same_digits(a, b):
    a = list(str(a))
    b = list(str(b))
    a.sort()
    b.sort()
    return a == b


def work():
    for digitcount in range(1, 10):
        for i in range(10 ** (digitcount - 1), 10 ** digitcount):
            ix6 = i * 6
            if ix6 > 10 ** digitcount:
                break

            if not same_digits(i, ix6):
                continue

            if not same_digits(i, i * 5):
                continue

            # in base 2, a n-bit-shift is multiplying by 2^n
            if not same_digits(i, i << 2):
                continue

            if not same_digits(i, i * 3):
                continue

            if not same_digits(i, i << 1):
                continue

            print(i)
            return

work()
