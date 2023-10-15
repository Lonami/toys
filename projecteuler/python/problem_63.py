def problem_definition():
    return '''The 5-digit number, 16807=7^5, is also a fifth power. Similarly, the 9-digit number,
    134217728=8^9, is a ninth power.

    How many n-digit positive integers exist which are also an nth power?'''


# Could highly be optimised, but it would be less simple
# First tried with n = [1..1000], power = [1..100]
# It can even be reduced to this:
count = 0
for n in range(1, 50):
    for power in range(1, 25):
        digits = len(str(n ** power))
        if digits > power:
            break

        if digits == power:
            count += 1

print(count)
