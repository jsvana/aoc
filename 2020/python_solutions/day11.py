import itertools
import sys
import time
from collections import defaultdict
from functools import lru_cache

highest_adapter = None

input_seats = []
with open(sys.argv[1]) as f:
    for line in f.readlines():
        line = line.strip()
        input_seats.append(list(line))


def seat_counts1(current_seats, x, y):
    counts = defaultdict(int)
    for i in [-1, 0, 1]:
        for j in [-1, 0, 1]:
            if i == 0 and j == 0:
                continue
            if (
                y + i < 0
                or x + j < 0
                or y + i >= len(current_seats)
                or x + j >= len(current_seats[0])
            ):
                continue

            counts[current_seats[y + i][x + j]] += 1

    return counts


def seat_counts2(current_seats, x, y):
    counts = defaultdict(int)
    for i in [-1, 0, 1]:
        for j in [-1, 0, 1]:
            if i == 0 and j == 0:
                continue

            multiple = 1
            while True:
                if (
                    y + (i * multiple) < 0
                    or x + (j * multiple) < 0
                    or y + (i * multiple) >= len(current_seats)
                    or x + (j * multiple) >= len(current_seats[0])
                ):
                    break

                if current_seats[y + (i * multiple)][x + (j * multiple)] == "#":
                    counts["#"] += 1
                    break
                elif current_seats[y + (i * multiple)][x + (j * multiple)] == "L":
                    counts["L"] += 1
                    break

                multiple += 1

    return counts


def advance(current_seats, count_func, overpopulation):
    next_seats = []
    for row in current_seats:
        next_seats.append(list(row))

    for y in range(len(current_seats)):
        for x in range(len(current_seats[y])):
            adjacents = count_func(current_seats, x, y)

            if current_seats[y][x] == "L" and adjacents["#"] == 0:
                next_seats[y][x] = "#"
            elif current_seats[y][x] == "#" and adjacents["#"] >= overpopulation:
                next_seats[y][x] = "L"
            else:
                next_seats[y][x] = current_seats[y][x]

    return next_seats


def same(old_seats, new_seats):
    for i in range(len(old_seats)):
        for j in range(len(old_seats[i])):
            if old_seats[i][j] != new_seats[i][j]:
                return False

    return True


def clone(other_seats):
    clone_seats = []
    for row in other_seats:
        clone_seats.append(list(row))
    return clone_seats


def loop_seats(seats, count_func, overpopulation):
    prev_seats = clone(seats)
    next_seats = advance(prev_seats, count_func, overpopulation)
    while not same(prev_seats, next_seats):
        prev_seats = clone(next_seats)
        next_seats = advance(next_seats, count_func, overpopulation)

    count = 0
    for i in range(len(next_seats)):
        for j in range(len(next_seats[i])):
            if next_seats[i][j] == "#":
                count += 1

    return count


print("Part 1: {}".format(loop_seats(input_seats, seat_counts1, 4)))
print("Part 2: {}".format(loop_seats(input_seats, seat_counts2, 5)))
