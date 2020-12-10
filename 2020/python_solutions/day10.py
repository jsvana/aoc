import sys
import time
from collections import defaultdict
import itertools
from functools import lru_cache

highest_adapter = None

adapters = []
with open(sys.argv[1]) as f:
    for line in f.readlines():
        line = line.strip()
        adapters.append(int(line))

adapters.sort()

highest_adapter = max(adapters) + 3

def part1(adapters, highest_adapter):
    differences = defaultdict(int)
    current_jolt = 0
    for adapter in adapters:
        difference = adapter - current_jolt
        if difference < 0:
            break

        if difference > 3:
            break

        differences[difference] += 1
        current_jolt = adapter

    difference = highest_adapter - current_jolt
    differences[difference] += 1
    current_jolt = highest_adapter

    return differences[3] * differences[1]

def make_graph(adapters):
    graph = defaultdict(set)

    adapters_set = set(adapters)
    adapters_set.add(highest_adapter)
    to_find = [0]
    visited = set()
    while to_find:
        start = to_find[0]
        to_find = to_find[1:]

        if start in visited:
            continue

        visited.add(start)

        for i in range(1, 4):
            possible = start + i
            if possible in adapters_set:
                graph[start].add(possible)
                to_find.append(possible)

    return graph


def part2(adapters, final):
    graph = make_graph(adapters)

    @lru_cache()
    def count_paths(start, final):
        if start == final:
            return 1

        total = 0
        for child in graph[start]:
            total += count_paths(child, final)

        return total

    return count_paths(0, highest_adapter)

print("Part 1: {}".format(part1(adapters, highest_adapter)))
print("Part 2: {}".format(part2(adapters, highest_adapter)))
