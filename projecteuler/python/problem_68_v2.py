class Node:
    def __init__(self, connections, head):
        self.value = 0

        self.head = head
        self.connections = connections

        # The path to follow by this node is the first not None value index
        self.path = None
        if head:
            for i, c in enumerate(connections):
                if c is not None:
                    self.path = i
                    break


class Gong:
    def __init__(self, nodes):
        self.nodes = nodes
        self.values = range(1, len(nodes)+1)

    def reset_values(self):
        for node in self.nodes:
            node.value = 0

    # region Drawing

    def draw(self):
        string = 'Nodes:\n'
        for node in self.nodes:
            if node.head:
                string = self.draw_node(string, node, node.path)

        return string

    def draw_node(self, string, node, path):
        connection = node.connections[path]
        if connection is None:
            string += '{}\n'.format(node.value)
        else:
            string += '{} -> '.format(node.value)
            string = self.draw_node(string, self.nodes[connection], path)

        return string

    # endregion

    def add_nodes(self, node, path=None):
        # If no path is given, this is a starting node
        if path is None:
            path = node.path

        connection = node.connections[path]
        if connection is None:
            return node.value
        else:
            return node.value + self.add_nodes(self.nodes[connection], path)

    def begin_try_solve(self, target):
        # Try solving for all heads
        available_values = [v for v in self.values]
        for node in self.nodes:
            if node.head:
                self.try_solve(target, head=node, available_values=available_values)


    def try_solve(self, target, head=None, node=None, available_values=None):
        # Start with the first node and all the values
        if node is None:
            node = head

        # Try all the available values for the first node
        for i, value in enumerate(available_values):
            node.value = value

            # (Step n) Check the count for this path
            # We may be over the target thus stop (and clear) and try another value
            if self.add_nodes(head) > target:
                node.value = 0
                continue

            connection = node.connections[head.path]
            if connection is None:
                # (Step end) There are no more connections in this path, check if we succeeded
                return self.add_nodes(head) == target
            else:
                # (Step 1) Follow the path, get a copy of the left values
                left_values = [available_values[j] for j in range(len(available_values)) if j != i]
                next_node = self.nodes[connection]
                if self.try_solve(target, head, next_node, left_values):
                    # (Step end ok) This solution is correct for this head
                    return True
                else:
                    # (Step end fail) This solution was not correct, try another starting value
                    continue


def get_test_gong():
    # Draw the connections clockwise and ending the row to understand these numbers
    return Gong([
        Node([1, None, None], True),
        Node([2, None, None], False),

        Node([None, 4, None], True),
        Node([None, 2, None], False),

        Node([None, None, 1], True),
        Node([None, None, 4], False)
    ])


get_test_gong().try_solve(9)
