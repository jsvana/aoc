import itertools
import sys
import time
from collections import defaultdict
import functools
import math
import numpy

with open(sys.argv[1]) as f:
    lines = f.readlines()
    earliest_start = int(lines[0])

    bus_times = []
    for t in lines[1].split(","):
        if t == "x":
            bus_times.append(t)
        else:
            bus_times.append(int(t))

def part1():
    lowest_time = None
    lowest_start = None
    for bus_time in bus_times:
        if bus_time == "x":
            continue

        mod = earliest_start % bus_time
        next_time = bus_time - mod
        if lowest_time is None or next_time < lowest_time:
            lowest_time = next_time
            lowest_start = bus_time

    return lowest_time * lowest_start

def chinese_remainder(n, a):
    sum = 0
    prod = functools.reduce(lambda a, b: a*b, n)
    for n_i, a_i in zip(n, a):
        p = prod // n_i
        sum += a_i * mul_inv(p, n_i) * p
    return sum % prod


def mul_inv(a, b):
    b0 = b
    x0, x1 = 0, 1
    if b == 1: return 1
    while a > 1:
        q = a // b
        a, b = b, a%b
        x0, x1 = x1 - q * x0, x0
    if x1 < 0: x1 += b0
    return x1


n = []
a = []
for i, time in enumerate(bus_times):
    if time == "x":
        continue

    n.append(time)
    a.append(-i)

print("Part 1: {}".format(part1()))
print("Part 2: {}".format(chinese_remainder(n, a)))
