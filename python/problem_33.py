from fractions import Fraction


def problem_definition():
    return '''The fraction 49/98 is a curious fraction, as an inexperienced mathematician in
    attempting to simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is
    obtained by cancelling the 9s.

    We shall consider fractions like, 30/50 = 3/5, to be trivial examples.

    There are exactly four non-trivial examples of this type of fraction, less than one in value,
    and containing two digits in the numerator and denominator.

    If the product of these four fractions is given in its lowest common terms, find the value of
    the denominator.'''


def can_be_obtained_cancelling(numerator, denominator):
    # Reduced
    redfra = Fraction(numerator, denominator)
    strnum = str(numerator)
    strden = str(denominator)

    # Cancelled
    cannum = int(strnum[0])
    canden = int(strden[1])
    if canden == 0 or strnum[1] != strden[0]:
        return
    canfra = Fraction(cannum, canden)

    return redfra == canfra


found = set()
for numerator in range(10, 100):
    for denominator in range(numerator + 1, 100):
        if can_be_obtained_cancelling(numerator, denominator):
            found.add(Fraction(numerator, denominator))


result = Fraction(1, 1)
for fraction in found:
    result *= fraction

print(result)
