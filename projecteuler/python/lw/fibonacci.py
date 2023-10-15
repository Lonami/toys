class Fib:
    """Iterator that yields numbers from the Fibonacci sequence.
    Note that this class does not yield the initial 0"""

    def __init__(self, maximum):
        self.max = maximum

    def __iter__(self):
        self.a = 0
        self.b = 1
        return self

    def __next__(self):
        if self.b > self.max:
            raise StopIteration

        self.a, self.b = self.b, self.a + self.b
        return self.a
