import sys
from collections import defaultdict

bag_map = {}

def sum_children(bags, start):
    children = bags.get(start, [])
    if not children:
        return 1

    bag_sum = 1
    for child in children:
        bag_sum += child["count"] * sum_children(bags, child["name"])

    return bag_sum

def contains_gold(bags, start):
    q = [start]
    visited = set()

    bag_sum = 0

    while q:
        n = q[0]
        q = q[1:]

        if n in visited:
            continue

        visited.add(n)

        for child in bags.get(n, []):
            if child["name"] == "shiny gold":
                return True

            q.append(child["name"])

    return False

with open(sys.argv[1]) as f:
    for line in f.readlines():
        line = line.strip()

        parts = line.split()
        outer = "{} {}".format(parts[0], parts[1])

        inners = line.split(" contain ")[1][:-1]

        if inners.startswith("no"):
            continue

        inner_names = []
        for inner in inners.split(", "):
            parts = inner.split()
            inner_names.append({
                "count": int(parts[0]),
                "name": "{} {}".format(parts[1], parts[2]),
            })

        bag_map[outer] = inner_names

    count = 0
    for name in bag_map:
        if contains_gold(bag_map, name):
            count += 1

    print("part 1: {}".format(count))

    print("part 2: {}".format(sum_children(bag_map, "shiny gold") - 1))
