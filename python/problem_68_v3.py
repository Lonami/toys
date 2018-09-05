class Node:
    def __init__(self, value=0, head=True, parent=None, next=None):
        self.value = value

        self.head = head
        self.parent = parent
        self.next = next

    def clone(self, depth, parent=None):
        clon = Node(self.value, parent is None, parent, None)
        # node.next should be None for this depth on this arm
        if depth > 1:
            clon.next = self.next.clone(depth-1, parent=clon)

        return clon

    def reset(self, depth):
        self.value = 0
        # node.next should be None for this depth on this arm
        if depth > 1:
            self.next.reset(depth-1)

    def get_value(self, depth):
        # node.next should be None for this depth on this arm
        if depth == 1:
            return self.value
        else:
            return self.value + self.next.get_value(depth-1)

    def set_next(self, node):
        node.parent = self
        self.next = node
        self.next.head = False

    def get_depth(self):
        if self.parent is None:
            return 1
        else:
            return 1 + self.parent.get_depth()

    def get_str(self, depth):
        # node.next should be None for this depth on this arm
        if depth == 1:
            return str(self.value)
        else:
            return '{} -> {}'.format(self.value, self.next.get_str(depth-1))


class Gong:
    def __init__(self, heads, arms_length, node_count):
        self.heads = heads
        self.arm_length = arms_length
        self.node_count = node_count


    def solve_heads(self, target):
        available_values = [v for v in range(1, self.node_count + 1)]
        for head in self.heads:
            print('Gonna solve.')
            self.solve_head(target, head)

    def solve_head(self, target, head):
        available_values = [v for v in range(1, self.node_count + 1)]
        for solved_node in self.solve_node(target, head, available_values=available_values, depth=self.arm_length):
            print(solved_node.get_str(depth=self.arm_length))

    def solve_node(self, target, head, node=None, available_values=None, depth=0):
        # Need to keep track of the depth since the end nodes are always the middle of another arm

        # Start with the head node
        if node is None:
            node = head

        # Try all the available values for the current node
        for i, value in enumerate(available_values):
            if (depth == 3 and value == 4 or
                depth == 2 and value == 3 or
                    depth == 1 and value == 2):
                a = 4+2 == 42  # Breakpoint me

            node.value = value

            # (Step n) Check the count for this path
            # We may be over the target thus stop (and clear) and try another value
            if head.get_value(self.arm_length) > target:
                node.value = 0
                continue

            # node.next should be None for this depth on this arm
            if depth == 1:
                # (Step end) There are no more connections in this path,
                # check if we succeeded and yield a clone of the success head
                if head.get_value(self.arm_length) == target:
                    yield head.clone(self.arm_length)
                    head.reset(self.arm_length)
                    break

            # We're not at the end of the arm yet
            elif depth > 1:
                # (Step 1) Follow the path, get a copy of the left_values
                left_values = [available_values[j] for j in range(len(available_values)) if j != i]

                # Calls are a bit weird since they are generators; but otherwise we wouldn't go in
                solved_nodes = [s for s in self.solve_node(target, head, node.next, left_values, depth-1)]
                if solved_nodes:
                    for s in solved_nodes:
                        yield s

                    # Break the whole chain as soon as we find a solution
                    break


def get_test_gong():
    node_count = 6
    nodes = [Node() for _ in range(node_count)]
    nodes[0].set_next(nodes[1])
    nodes[1].set_next(nodes[2])

    nodes[3].set_next(nodes[2])
    nodes[2].set_next(nodes[4])

    nodes[5].set_next(nodes[4])
    nodes[4].set_next(nodes[1])

    return Gong([nodes[0], nodes[3], nodes[4]], 3, node_count)


get_test_gong().solve_heads(9)
