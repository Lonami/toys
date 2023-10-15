# <editor-fold desc="Card class">
class Card:
    encoded_to_name = {
        'C': 'Clubs',
        'D': 'Diamonds',
        'H': 'Hearts',
        'S': 'Spades'
    }
    encoded_to_value = {
        '2': 2,
        '3': 3,
        '4': 4,
        '5': 5,
        '6': 6,
        '7': 7,
        '8': 8,
        '9': 9,
        'T': 10,
        'J': 11,  # Jack
        'Q': 12,  # Queen
        'K': 13,  # King
        'A': 14   # Ace
    }

    def __init__(self, encoded):
        self.evalue = encoded[0]
        self.suit = encoded[1]

        self.value = Card.encoded_to_value[self.evalue]
        self.name = Card.encoded_to_name[self.suit]
        pass

    def __repr__(self):
        return "'{}{}'".format(self.evalue, self.suit)

    def __str__(self):
        return '{} of {}'.format(self.value, self.name)

# </editor-fold>


def same_suit(cards):
    """Determines whether ALL the cards are of the same suit"""
    return all(cards[0].suit == cards[i].suit for i in range(1, 5))


def are_consecutive(cards):
    """Determines whether the values of the cards are consecutive"""
    cards = sorted(cards, key=lambda c: c.value)
    return all(cards[i].value+1 == cards[i+1].value for i in range(4))


# <editor-fold desc="Is something (from higher to lower)">


# Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
def is_royal_flush(cards):
    return all(c.value in range(10, 14+1) for c in cards) and same_suit(cards)


# Straight Flush: All cards are consecutive values of same suit.
def is_straight_flush(cards):
    return are_consecutive(cards) and same_suit(cards)


# Four of a Kind: Four cards of the same value.
def is_four_of_a_kind(cards):
    return is_n_of_a_kind(cards, 4)


# Full House: Three of a kind and a pair.
def is_full_house(cards):
    return is_three_of_a_kind(cards) and is_one_pair(cards)


# Flush: All cards of the same suit.
def is_flush(cards):
    return same_suit(cards)


# Straight: All cards are consecutive values.
def is_straight(cards):
    return are_consecutive(cards)


# Three of a Kind: Three cards of the same value.
def is_three_of_a_kind(cards):
    return is_n_of_a_kind(cards, 3)


# Two Pairs: Two different pairs.
def is_two_pairs(cards):
    return len(get_pairs_values(cards)) == 2


# One Pair: Two cards of the same value.
def is_one_pair(cards):
    return len(get_pairs_values(cards)) == 1


# High Card: Highest value card.
def is_high_card(cards):
    return max(c.value for c in cards)


# </editor-fold>

# <editor-fold desc="values">


# Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
def value_royal_flush(cards):
    return 1 if is_royal_flush(cards) else 0


# Straight Flush: All cards are consecutive values of same suit.
def value_straight_flush(cards):
    return value_high_card(cards) if is_straight_flush(cards) else 0


# Four of a Kind: Four cards of the same value.
def value_four_of_a_kind(cards):
    return value_n_of_a_kind(cards, 4) if is_four_of_a_kind(cards) else 0


# Full House: Three of a kind and a pair.
def value_full_house(cards):
    return value_three_of_a_kind(cards) if is_full_house(cards) else 0


# Flush: All cards of the same suit.
def value_flush(cards):
    return value_high_card(cards) if is_flush(cards) else 0


# Straight: All cards are consecutive values.
def value_straight(cards):
    return value_high_card(cards) if is_straight(cards) else 0


# Three of a Kind: Three cards of the same value.
def value_three_of_a_kind(cards):
    return value_n_of_a_kind(cards, 3) if is_three_of_a_kind(cards) else 0


# Two Pairs: Two different pairs.
def value_two_pairs(cards):
    return value_pairs(cards) if is_two_pairs(cards) else 0


# One Pair: Two cards of the same value.
def value_one_pair(cards):
    return value_pairs(cards) if is_one_pair(cards) else 0


# High Card: Highest value card.
def value_high_card(cards):
    return max(c.value for c in cards)


# </editor-fold>

# <editor-fold desc="Is something and value utils">


def get_pairs_values(cards):
    return [value for value, times in get_value_count(cards).items() if times == 2]


def value_pairs(cards):
    pairs = get_pairs_values(cards)
    return max(p for p in pairs) if pairs else 0


def is_n_of_a_kind(cards, n):
    for times in get_value_count(cards).values():
        if times == n:
            return True
    return False


def value_n_of_a_kind(cards, n):
    for value, times in get_value_count(cards).items():
        if times == n:
            return value
    return 0


def get_value_count(cards):
    """Returns a dictionary containing how many of each value there are"""
    #                             2..Ace+1 (since its exclusive)
    values = {v: 0 for v in range(2, 14+1)}
    for c in cards:
        values[c.value] += 1
    return values


# </editor-fold>


def problem_definition():
    return '''In the card game poker, a hand consists of five cards and are ranked, from lowest to highest,
    in the following way:

        High Card: Highest value card.
        One Pair: Two cards of the same value.
        Two Pairs: Two different pairs.
        Three of a Kind: Three cards of the same value.
        Straight: All cards are consecutive values.
        Flush: All cards of the same suit.
        Full House: Three of a kind and a pair.
        Four of a Kind: Four cards of the same value.
        Straight Flush: All cards are consecutive values of same suit.
        Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.

    The cards are valued in the order:
    2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.

    If two players have the same ranked hands then the rank made up of the highest value wins;
    for example, a pair of eights beats a pair of fives (see example 1 below). But if two ranks tie,
    for example, both players have a pair of queens, then highest cards in each hand are compared
    (see example 4 below); if the highest cards tie then the next highest cards are compared, and so on.

    Consider the following five hands dealt to two players:
    ┌──────┬─────────────────────┬───────────────────────┬──────────┐
    │ Hand │    Player 1         │   Player 2            │  Winner  │
    │──────┼─────────────────────┼───────────────────────┼──────────┤
    │  1.  │ 5H 5C 6S 7S KD      │ 2C 3S 8S 8D TD        │ Player 2 │
    │      │ > Pair of Fives     │ > Pair of Eights      │          │
    │──────┼─────────────────────┼───────────────────────┼──────────┤
    │  2.  │ 5D 8C 9S JS AC      │ 2C 5C 7D 8S QH        │ Player 1 │
    │      │ > Highest card Ace  │ > Highest card Queen  │          │
    │──────┼─────────────────────┼───────────────────────┼──────────┤
    │  3.  │ 2D 9C AS AH AC      │ 3D 6D 7D TD QD        │ Player 2 │
    │      │ > Three Aces        │ > Flush with Diamonds │          │
    │──────┼─────────────────────┼───────────────────────┼──────────┤
    │      │ 4D 6S 9H QH QC      │ 3D 6D 7H QD QS        │          │
    │  4.  │ > Pair of Queens    │ > Pair of Queens      │ Player 1 │
    │      │ > Highest card Nine │ > Highest card Seven  │          │
    │──────┼─────────────────────┼───────────────────────┼──────────┤
    │      │ 2H 2D 4C 4D 4S      │ 3C 3D 3S 9S 9D        │          │
    │  5.  │ > Full House        │ > Full House          │ Player 1 │
    │      │ > With Three Fours  │ > With Three Threes   │          │
    └──────┴─────────────────────┴───────────────────────┴──────────┘

    The file, poker.txt, contains one-thousand random hands dealt to two players. Each line of the file contains
    ten cards (separated by a single space): the first five are Player 1's cards and the last five are Player 2's
    cards. You can assume that all hands are valid (no invalid characters or repeated cards), each player's hand
    is in no specific order, and in each hand there is a clear winner.

    How many hands does Player 1 win?'''


max_name_length = 21  # i.e. len('Four of a Kind (12)') == 19 + padding


def get_cards(hand, index=0):
    return [Card(hand[i:i+2]) for i in range(15 * index, 15 * (index + 1), 3)]


def get_hand_for_print(hand, center=True):
    result = str(hand).replace("'", '').replace(',', '')
    if center:
        return result.center(max_name_length)
    return result


names = ('Royal flush',      # 0
         'Straight flush',   # 1
         'Four of a kind',   # 2
         'Full house',       # 3
         'Flush',            # 4
         'Straight',         # 5
         'Three of a kind',  # 6
         'Two pairs',        # 7
         'One pair',         # 8
         'High card')        # 9


values = (value_royal_flush,      # 0
          value_straight_flush,   # 1
          value_four_of_a_kind,   # 2
          value_full_house,       # 3
          value_flush,            # 4
          value_straight,         # 5
          value_three_of_a_kind,  # 6
          value_two_pairs,        # 7
          value_one_pair,         # 8
          value_high_card)        # 9

fallback_values = (value_high_card,)


def determine_winner(player1, player2):
    for i in range(10):
        value = values[i]

        value1 = value(player1)
        value2 = value(player2)
        if value1 == value2:
            if value1 == 0:
                continue
            else:
                # Same rank on this score? Then highest value
                value1 = value_high_card(player1)
                value2 = value_high_card(player2)
                reason = "{}, {} vs {}".format(names[i], value1, value2)
        else:
            reason = names[i]

        return (1, reason) if value1 > value2 else (2, reason)


total1wins = 0
total2wins = 0
hand = 0


print('┌──────┬─────────────────────┬─────────────────────┬──────────┐')
print('│ Hand │      Player  1      │      Player  2      │  Winner  │')
with open('resources/p054_poker.txt', encoding='utf-8') as file:
    for line in file:
        hand += 1
        player1 = get_cards(line, 0)
        player2 = get_cards(line, 1)
        winner, reason = determine_winner(player1, player2)
        if winner == 1:
            total1wins += 1
        else:
            total2wins += 1

        print('│──────┼─────────────────────┼─────────────────────┼──────────┤')
        print('│{}│{}│{}│ Player {} │ {}'
              .format(str(hand).center(6),
                      get_hand_for_print(player1),
                      get_hand_for_print(player2),
                      winner,
                      reason))
print('└──────┴─────────────────────┴─────────────────────┴──────────┘')

print('Player 1 won {} times and player 2 won {}'.format(total1wins, total2wins))
