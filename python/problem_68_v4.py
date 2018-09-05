class Node:
    def __init__(self, value=0):
        self.value = value

    def reset(self):
        self.value = 0

    def clone(self):
        return Node(self.value)


class GongArm:
    def __init__(self, nodes):
        self.nodes = nodes

    def clone(self):
        return GongArm([n.clone for n in self.nodes])


class Gong:
    def __init__(self, layout):
        """A list of n ≥ 3 lists with 3 elements should be given with gong shape"""
        self.layout = layout

        # By the shape of the gong, the last arm and first element will be the highest value
        self.nodes = [Node() for _ in range(layout[-1][0]+1)]

    def get_arm_value(self, arm):
        return sum(self(arm, i) for i in range(3))

    def __call__(self, arm, i):
        return self.layout[arm][i]

def get_test_gong():
    return Gong([
        [0, 1, 2],
        [3, 2, 4],
        [5, 4, 1]
    ])

def solve_gong(gong, target, arm, i=0, available_values=None):
    """Solves the given gong (may have other arms used)
       at the given arm and yields possible solutions,
       seeking target as the value summed by the arm"""

    if available_values is None:
        available_values = [v for v in range(1, len(gong.nodes) + 1)]

    for value in available_values:
        gong(arm, i) =



gong = get_test_gong()
print('Nice')
