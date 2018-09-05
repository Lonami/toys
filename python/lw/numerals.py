# Class made by Lonami Exo (c) LonamiWebs
# Creation date: 07-07-2016


class NotIntegerError(ValueError):
    """Defines a not an integer error"""


class NotDigitError(ValueError):
    """Defines a not an integer error"""


class OutOfRangeError(ValueError):
    """Defines an out of range error.
    Should be raised when the value is too high or too low."""


__known_int_to_word = {
    0: 'zero',  10: 'ten',
    1: 'one',   11: 'eleven',
    2: 'two',   12: 'twelve',    20: 'twenty',
    3: 'three', 13: 'thirteen',  30: 'thirty',
    4: 'four',  14: 'fourteen',  40: 'forty',
    5: 'five',  15: 'fifteen',   50: 'fifty',
    6: 'six',   16: 'sixteen',   60: 'sixty',
    7: 'seven', 17: 'seventeen', 70: 'seventy',
    8: 'eight', 18: 'eighteen',  80: 'eighty',
    9: 'nine',  19: 'nineteen',  90: 'ninety'
}


def to_english_numeral(value):
    """Converts an integer value to English string numerals"""
    # Sanity checks
    if not isinstance(value, int):
        raise NotIntegerError('A non-integer value was provided')

    if not -10000 < value < 10000:
        raise OutOfRangeError('The value must be between -9999 and 9999')

    # If it's a known-value, return it straightforward!
    if value in __known_int_to_word:
        return __known_int_to_word[value]

    # If it's a negative value, append 'minus '
    if value < 0:
        result = 'minus '
        value = -value
    else:
        result = ''

    # Extract the digits from the original value
    digits = [int(d) for d in list(str(value))]
    digitcount = len(digits)

    # Append thousands, if necessary
    if digitcount >= 4 and digits[-4] != 0:
        result += to_english_numeral(digits[-4]) + ' thousand '

    # Append hundreds, if necessary
    if digitcount >= 3 and digits[-3] != 0:
        result += to_english_numeral(digits[-3]) + ' hundred '

    # Append tenths, if necessary
    if digitcount >= 2 and digits[-2] != 0:
        # If the value was greater than 100, append 'and '
        if value > 100:
            result += 'and '

        # Check if we're under 20, then we have an specific word!
        if digits[-2] < 2:
            tenths = int(str(value)[-2:])
            result += __known_int_to_word[tenths]
            return result  # Return, as we don't need to check first digits any more

        # Otherwise, we're over 20, so we need that plus an hyphen
        tenths = digits[-2] * 10
        result += __known_int_to_word[tenths]

        # If the last digit is non-zero, append an hyphen
        if digits[-1] != 0:
            result += '-'

    # Append zero digit if and only if it's alone
    if digitcount == 1 and digits[-1] == 0:
        result += to_english_numeral(digits[-1])

    # Else, append digit if necessary
    elif digitcount >= 1 and digits[-1] != 0:
        # If we haven't yet appended 'and ', now we should (if the user wants)!
        if 'and' not in result and digits[-2] == 0 and value > 100:
            result += 'and '
        result += to_english_numeral(digits[-1])

    return result.strip()
