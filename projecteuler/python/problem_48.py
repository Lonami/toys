def problem_definition():
    return '''The series, 1^1 + 2² + 3³ + ... + 10^10 = 10405071317.
    Find the last ten digits of the series, 1^1 + 2² + 3³ + ... + 1000^1000.'''

print(str(sum(i**i for i in range(1, 1000 + 1)))[-10:])
