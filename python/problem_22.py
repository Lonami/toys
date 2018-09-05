def problem_definition():
    return '''Using names.txt (right click and 'Save Link/Target As...'), a 46K text file
    containing over five-thousand first names, begin by sorting it into alphabetical order.
    Then working out the alphabetical value for each name, multiply this value by its alphabetical
    position in the list to obtain a name score.

    For example, when the list is sorted into alphabetical order, COLIN, which is worth
    3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list.
    So, COLIN would obtain a score of 938 × 53 = 49714.

    What is the total of all the name scores in the file?'''


def score(name):
    result = 0
    for c in name:
        result += ord(c) - 64  # A is 65 in ASCII

    return result

totalscore = 0
with open('resources/p022_names.txt') as file:
    names = [name.strip('"') for name in file.readline().split(',')]
    names.sort()

    # name index
    namei = 1
    for n in names:
        totalscore += score(n) * namei
        namei += 1

print(totalscore)
