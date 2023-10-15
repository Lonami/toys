def problem_definition():
    return '''By starting at the top of the triangle below and moving to adjacent numbers
    on the row below, the maximum total from top to bottom is 23.

                                                 <3>
                                               <7>  4
                                              2  <4>  6
                                            8   5  <9>  3

    That is, 3 + 7 + 4 + 9 = 23.

    Find the maximum total from top to bottom in triangle.txt (right click and 'Save Link/Target As...'),
    a 15K text file containing a triangle with one-hundred rows.

    NOTE: This is a much more difficult version of Problem 18.
    It is not possible to try every route to solve this problem, as there are 2^99 altogether!
    If you could check one trillion (10^12) routes every second it would take over twenty
    billion years to check them all. There is an efficient algorithm to solve it. ;o)'''


# Load the triangle
with open('resources/p067_triangle.txt', encoding='utf-8') as file:
    triangle = [[int(n) for n in line.split()] for line in file]


# This time instead brute forcing we'll start from the bottom of the triangle
# Each superior layer will contain it's previous value + the highest below it
#
# We do this because once we get to this layer, we'll pick the greatest value
# on the next one, so the smallest gets "erased", and we end with the highest


# Iterate over the triangle in reverse order
# - 2 to skip the latest layer
for i in range(len(triangle) - 2, -1, -1):

    # Iterate over the current layer
    for j in range(len(triangle[i])):
        # Get the value below this layer, on the left and on the right
        below_left = triangle[i + 1][j + 0]
        below_right = triangle[i + 1][j + 1]

        # Add the highest to this layer
        triangle[i][j] += below_left if below_left > below_right else below_right


# Now print the first value (at the top), which will be the highest
# since we kept adding only the highest values
print(triangle[0][0])
