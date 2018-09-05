def problem_definition():
    return '''The cube, 41063625 (345³), can be permuted to produce two other cubes:
    56623104 (384³) and 66430125 (405³). In fact, 41063625 is the smallest cube which has exactly
    three permutations of its digits which are also cube.

    Find the smallest cube for which exactly five permutations of its digits are cube.'''

#
#  -- Made by Lonami (c) LonamiWebs | the 14-07-2016 --
#  ================== HOW THIS WORKS ==================
#
# Instead calculating every permutation, we calculate the only one which is sorted.
# This way, we avoid checking many permutations to determine whether they're cubes.
#
# This mentioned permutation is stored in a dictionary, which keeps track of how many times
# it has been calculated by any other cube. Then it's matter of checking whether how many
# cubes yielded that permutation. If we reached our goal, we can then stop.
#
# Another dictionary keeps track of {sorted value: [original values]}, for later
# printing of all the cubes (and the cube root) of those which caused the permutation

sorted_cubes = {}
sorted_to_original = {}
goal = 5
break_on_goal = True
for i in range(1, 100000):
    # Calculate the cube and its sorted value
    cube = str(i ** 3)
    sorted_cube = ''.join(sorted(cube))

    # First keep track of the original
    if sorted_cube in sorted_to_original:
        sorted_to_original[sorted_cube].append((cube, i))
    else:
        sorted_to_original[sorted_cube] = [(cube, i)]

    # Then check if this permutation (sorted permutation) was already found
    if sorted_cube in sorted_cubes:
        sorted_cubes[sorted_cube] += 1

        # If we reached the goal, yay! Print the originals too
        if sorted_cubes[sorted_cube] >= goal:
            print('Found {} so far for {} (largest). Originals:'.format(sorted_cubes[sorted_cube], cube))
            for original, sqrt3 in sorted_to_original[sorted_cube]:
                print('-> {} ({}³)'.format(original, sqrt3))

            if break_on_goal:
                break
    else:
        sorted_cubes[sorted_cube] = 1


# This old method checks every permutation to determine whether it's a cube or not.
# If n permutations are cubes, problem solved! However, this is cumbersome.
'''
# Cache some cubes and its inverses
max_cube = 5000
cubes = set(i ** 3 for i in range(1, max_cube * 100))

# Start at 346 since there's no point on checking below :)
max_permutations = 0
# for i in range(345, max_cube):
for i in range(3000, max_cube):
    count = 0
    cube = str(i ** 3)
    if i % 10 == 0:
        print('Working with {} ({}/{})...'.format(cube, i, max_cube))

    for p in unique_permutations(cube):
        if p[0] == '0':  # discard if it starts at 0
            continue

        if int(''.join(p)) in cubes:
            count += 1

    if count > max_permutations:
        print('Found new max of {} permutations for {}³. Valid permutations:'.format(count, i))
        for p in unique_permutations(cube):
            if p[0] == '0':
                continue
            if int(''.join(p)) in cubes:
                print('-> {}'.format(''.join(p)))

        max_permutations = count

# Next improvement:
# Generate a bunch of permutations to avoid duplicates... I guess
'''
