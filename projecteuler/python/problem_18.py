def problem_definition():
    return '''By starting at the top of the triangle below and moving to adjacent numbers
    on the row below, the maximum total from top to bottom is 23.

                                                 <3>
                                               <7>  4
                                              2  <4>  6
                                            8   5  <9>  3

    That is, 3 + 7 + 4 + 9 = 23.

    Find the maximum total from top to bottom of the triangle below:

                                                75
                                              95  64
                                            17  47  82
                                          18  35  87  10
                                        20  04  82  47  65
                                      19  01  23  75  03  34
                                    88  02  77  73  07  63  67
                                  99  65  04  28  06  16  70  92
                                41  41  26  56  83  40  80  70  33
                              41  48  72  33  47  32  37  16  94  29
                            53  71  44  65  25  43  91  52  97  51  14
                          70  11  33  28  77  73  17  78  39  68  17  57
                        91  71  52  38  17  14  91  43  58  50  27  29  48
                      63  66  04  68  89  53  67  30  73  16  69  87  40  31
                    04  62  98  27  23  09  70  98  73  93  38  53  60  04  23

    NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route.
    However, Problem 67, is the same challenge with a triangle containing one-hundred rows;
    it cannot be solved by brute force, and requires a clever method! ;o)'''


triangle = [
    [75],
    [95, 64],
    [17, 47, 82],
    [18, 35, 87, 10],
    [20,  4, 82, 47, 65],
    [19,  1, 23, 75,  3, 34],
    [88,  2, 77, 73,  7, 63, 67],
    [99, 65,  4, 28,  6, 16, 70, 92],
    [41, 41, 26, 56, 83, 40, 80, 70, 33],
    [41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
    [53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
    [70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
    [91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
    [63, 66,  4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
    [ 4, 62, 98, 27, 23,  9, 70, 98, 73, 93, 38, 53, 60,  4, 23]
]


maxfound = 0


def dorecurse():
    recurse(0, 0, 0, '')


# ati stands for "at index"
# lvalue/lpath stand for "last value/path"
def recurse(index, ati, lvalue, lpath):

    # We can't check more
    if index == len(triangle):
        return

    # current sum value at this point and path
    cvalue = lvalue + triangle[index][ati]
    if len(lpath) == 0:
        cpath = str(triangle[index][ati])
    else:
        cpath = lpath + '->' + str(triangle[index][ati])

    # nexti stands for "next index"
    nexti = index + 1

    # if the next index isn't equal to the triangle length, we can check more
    can_check_more = nexti != len(triangle)

    if can_check_more:
        can_go_left = ati > 0
        can_go_right = ati < len(triangle[index + 1])

        if can_go_left:
            recurse(nexti, ati + 0, cvalue, cpath)
        if can_go_right:
            recurse(nexti, ati + 1, cvalue, cpath)

    else:  # can't check more, we're done
        global maxfound
        if cvalue > maxfound:
            maxfound = cvalue
            print('New max of {}: {}'.format(maxfound, cpath))


dorecurse()
