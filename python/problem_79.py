def problem_definition():
    return '''A common security method used for online banking is to ask the user
    for three random characters from a passcode. For example, if the passcode was 531278,
    they may ask for the 2nd, 3rd, and 5th characters; the expected reply would be: 317.

    The text file, keylog.txt, contains fifty successful login attempts.

    Given that the three characters are always asked for in order, analyse the file so as
    to determine the shortest possible secret passcode of unknown length.'''


# [b, c, ...] must be after a
afters = {}
for i in range(10):
    afters[i] = set()

appeared = set()


# Load which digits should come after which
with open('resources/p079_keylog.txt') as file:
    for line in file:
        a = int(line[0])
        for i in range(1, len(line)-1):
            b = int(line[i])
            afters[a].add(b)

            appeared.add(a)
            appeared.add(b)

# Find which did not appear
non_appeared = set(i for i in range(10)) - appeared
print('These numbers did not appear:', non_appeared)


none_after = set()


def check_digit(a):
    bs = afters[a]
    if bs:
        # If there is any digit, check if those digits which must come after
        # contain this digit
        for b in bs:
            if b == a:
                pass
    else:
        none_after.add(a)

# Start analysing all the digits
for a in afters.keys():
    check_digit(a)

# Verbose
for a, b in afters.items():
    if b:
        print('After {} there must be {}'.format(a, b))
    else:
        print('There must be none after {}'.format(a))
