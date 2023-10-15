def problem_definition():
    return '''Take the number 192 and multiply it by each of 1, 2, and 3:
                                    192 × 1 = 192
                                    192 × 2 = 384
                                    192 × 3 = 576

    By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576 the
    concatenated product of 192 and (1,2,3)

    The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the pandigital,
    918273645, which is the concatenated product of 9 and (1,2,3,4,5).

    What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of
    an integer with (1,2, ... , n) where n > 1?'''


# Courtesy of problem_32.py
def is_pandigital(nstr):
    return (len(nstr) == 9 and
            len(set(nstr)) == 9 and
            '0' not in nstr)


def get_pandigital_or_zero(n):
    concatenated_products = ''
    m = 0

    # While length is less than 10, it can be pandigital!
    while len(concatenated_products) < 10:
        m += 1
        concatenated_products += str(n * m)
        if is_pandigital(concatenated_products):
            return int(concatenated_products)

    return 0


def print_test(n):
    m = 0
    result = ''
    while not is_pandigital(result):
        m += 1
        result += str(n * m)
        print('{} x {} = {}'.format(n, m, n * m))
    print(result)


max_pandigital = 0
for number in range(1, 10000):
    pandigital = get_pandigital_or_zero(number)
    if pandigital > max_pandigital:
        print('Found new highest: {}'.format(pandigital))
        max_pandigital = pandigital
