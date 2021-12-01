from collections import defaultdict
import re
import sys


class Node:

    def __init__(self, name, weight):
        self.name = name
        self.weight = weight
        self.parent = None
        self.children = set()
        self.height = -1
        self.child_weights = {}

    @property
    def overall_weight(self):
        return self.weight + sum(self.child_weights.values())

    @property
    def unbalanced(self):
        if not self.children:
            return False
        return len(set(self.child_weights.values())) != 1


def determine_heights(nodes, name):
    if not nodes[name].children:
        nodes[name].height = 0
        return 0

    heights = []
    for child in nodes[name].children:
        if nodes[child].height == -1:
            nodes[child].height = determine_heights(nodes, child)
        heights.append(nodes[child].height)
    nodes[name].height = max(heights) + 1
    return nodes[name].height


def set_child_weight(nodes, name):
    for child in nodes[name].children:
        if not nodes[child].child_weights:
            set_child_weight(nodes, child)

        weight = nodes[child].weight + sum(nodes[child].child_weights.values())
        nodes[name].child_weights[child] = weight


def main():
    nodes = {}
    with open(sys.argv[1], 'r') as f:
        for line in f.readlines():
            line = line.strip()
            m = re.match(r'([a-z]+) \((\d+)\)', line)
            nodes[m.group(1)] = Node(m.group(1), int(m.group(2)))

            if ' -> ' not in line:
                continue

            nodes[m.group(1)].children.update(
                line.split(' -> ')[1].split(', '),
            )
    for node in nodes.values():
        for child in node.children:
            nodes[child].parent = node.name

    root_name = None
    for node in nodes.values():
        if node.parent is None:
            root_name = node.name

    determine_heights(nodes, root_name)

    set_child_weight(nodes, root_name)
    min_height = float('inf')
    min_node = None
    for name, node in nodes.items():
        if node.unbalanced and node.height < min_height:
            min_height = node.height
            min_node = name

    weights = list(set(nodes[min_node].child_weights.values()))
    if len(weights) != 2:
        print('wat', len(weights))
        sys.exit(1)

    weight_diff = abs(weights[0] - weights[1])
    weight_counts = defaultdict(int)
    for weight in nodes[min_node].child_weights.values():
        weight_counts[weight] += 1

    weight_to_find = None
    for weight, count in weight_counts.items():
        if count == 1:
            weight_to_find = weight

    for name, weight in nodes[min_node].child_weights.items():
        if weight == weight_to_find:
            print(nodes[name].weight - weight_diff)


main()
