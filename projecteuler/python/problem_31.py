def problem_definition():
    return '''In England the currency is made up of pound, £, and pence, p, and there are eight
    coins in general circulation:

        1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).

    It is possible to make £2 in the following way:

        1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p

    How many different ways can £2 be made using any number of coins?'''


# We must start with the 200 coin
# Then with the 100, we can also use another of 100
# And also with 100, we can also use two of 50
# Carrying on w/100, we can also use one of 50, two of 20, one of 10...
# We can never use a bigger coin than the last.
# For example, if we use the 2 coin, we cannot then use the 50 coin; that's been used already

def do_work(oldsum, available_coins):
    global okcount

    index = 0
    for coin in available_coins:
        newsum = oldsum + coin

        # Nailed it! Add one to the ok count
        if newsum == 200:
            okcount += 1

        # We can still check. However, slice the list to avoid repeating bigger coins!
        elif newsum < 200:
            do_work(newsum, available_coins[index:])

        # Increment the index, later used to skip the used coins
        index += 1


okcount = 0
do_work(0, [200, 100, 50, 20, 10, 5, 2, 1])
print(okcount)
