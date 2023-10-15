# Class made by Lonami Exo (c) LonamiWebs
# Creation date: 13-07-2016


class Collatz:
    """Iterator that yields numbers from the Collatz sequence:
    n → n/2 (n is even)
    n → 3n + 1 (n is odd)"""

    def __init__(self, start_at):
        self.start_at = start_at

    def __iter__(self):
        self.n = self.start_at
        return self

    def __next__(self):
        # Store current
        c = self.n
        if c == 0:
            raise StopIteration

        # Calculate next
        if self.n == 1:  # set to 0 to stop next time
            self.n = 0

        elif self.n % 2 == 0:  # even -> n/2
            # Since computers work in binary, dividing by 2 equals shifting one to the left
            # To understand why, think about powers in base 10 and what means to shift
            self.n >>= 1

        else:  # odd -> 3n + 1
            self.n = 3 * self.n + 1

        return c
