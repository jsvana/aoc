import itertools
import sys
import time
from collections import defaultdict
from functools import lru_cache
import math

path = []
with open(sys.argv[1]) as f:
    for line in f.readlines():
        line = line.strip()
        path.append((line[0], int(line[1:])))

def follow_path1(input_path):
    x = 0
    y = 0
    direction = (1, 0)
    left_directions_order = [(1, 0), (0, -1), (-1, 0), (0, 1)]

    for face, magnitude in input_path:
        if face == 'F':
            x += direction[0] * magnitude
            y += direction[1] * magnitude
        elif face == 'L':
            magnitude //= 90
            pos = left_directions_order.index(direction)
            direction = left_directions_order[(pos + magnitude) % 4]
        elif face == 'R':
            magnitude //= 90
            pos = left_directions_order.index(direction)
            direction = left_directions_order[(pos - magnitude) % 4]
        elif face == 'N':
            temp_direction = (0, -1)
            x += temp_direction[0] * magnitude
            y += temp_direction[1] * magnitude
        elif face == 'S':
            temp_direction = (0, 1)
            x += temp_direction[0] * magnitude
            y += temp_direction[1] * magnitude
        elif face == 'E':
            temp_direction = (1, 0)
            x += temp_direction[0] * magnitude
            y += temp_direction[1] * magnitude
        elif face == 'W':
            temp_direction = (-1, 0)
            x += temp_direction[0] * magnitude
            y += temp_direction[1] * magnitude

    return abs(x) + abs(y)

def rotate(x, y, degrees):
    radians = degrees * math.pi / 180

    return (
        x * int(math.cos(radians)) - y * int(math.sin(radians)),
        x * int(math.sin(radians)) + y * int(math.cos(radians)),
    )

def follow_path2(input_path):
    x = 0
    y = 0
    waypoint_x = 10
    waypoint_y = 1

    for face, magnitude in input_path:
        if face == 'F':
            x += waypoint_x * magnitude
            y += waypoint_y * magnitude
        elif face == 'L':
            waypoint_x, waypoint_y = rotate(waypoint_x, waypoint_y, magnitude)
        elif face == 'R':
            waypoint_x, waypoint_y = rotate(waypoint_x, waypoint_y, -magnitude)
        elif face == 'N':
            waypoint_y += magnitude
        elif face == 'S':
            waypoint_y -= magnitude
        elif face == 'E':
            waypoint_x += magnitude
        elif face == 'W':
            waypoint_x -= magnitude

    return abs(x) + abs(y)

print("Part 1: {}".format(follow_path1(path)))
print("Part 2: {}".format(follow_path2(path)))
