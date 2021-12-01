import re
import sys


class Node:

    def __init__(self, name, weight):
        self.name = name
        self.weight = weight
        self.parents = set()
        self.children = set()


def main():
    nodes = {}
    with open(sys.argv[1], 'r') as f:
        for line in f.readlines():
            line = line.strip()
            m = re.match(r'([a-z]+) \((\d+)\)', line)
            nodes[m.group(1)] = Node(m.group(1), m.group(2))

            if ' -> ' not in line:
                continue

            nodes[m.group(1)].children.update(
                line.split(' -> ')[1].split(', '),
            )
    for node in nodes.values():
        for child in node.children:
            nodes[child].parents.add(node.name)

    for node in nodes.values():
        if not node.parents:
            print(node.name)


main()
