def problem_definition():
    return '''The nth term of the sequence of triangle numbers is given by, tn = (1/2)*n(n+1);
    so the first ten triangle numbers are:

        1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

    By converting each letter in a word to a number corresponding to its alphabetical position and adding
    these values we form a word value. For example, the word value for SKY is 19 + 11 + 25 = 55 = t10.
    If the word value is a triangle number then we shall call the word a triangle word.

    Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly
    two-thousand common English words, how many are triangle words?'''


def char_value(c):
    return ord(c) - 64  # - ord('A') == 65


def word_value(word):
    return sum(char_value(c) for c in word)


def triangle(n):
    return int(0.5 * n * (n + 1))


def triangle_word(word):
    wv = word_value(word)
    i = 0
    while True:
        i += 1
        t = triangle(i)
        if t > wv:
            return False
        if t == wv:
            return True


count = 0
with open('resources/p042_words.txt', encoding='utf-8') as words:
    for word in words:
        word = word.rstrip()
        if triangle_word(word):
            print(word)
            count += 1

print(count)
