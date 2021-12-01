from enum import Enum
import math
import sys


class Quadrant(Enum):
    BOTTOM = 0
    LEFT = 1
    TOP = 2
    RIGHT = 3


class Square:

    def __init__(self, root, square):
        self.root = root
        self.square = square
        self.quadrants = {
            Quadrant.BOTTOM: (square - root + 1, square),
            Quadrant.LEFT: (square - 2 * (root - 1), square - root),
            Quadrant.TOP: (
                square - 3 * (root - 1),
                square - 2 * (root - 1) - 1,
            ),
            Quadrant.RIGHT: (
                square - 4 * (root - 1) + 1,
                square - 3 * (root - 1) - 1,
            ),
        }

    def quadrant(self, number):
        for k, v in self.quadrants.items():
            if number >= v[0] and number <= v[1]:
                return k
        raise ValueError('{} not in this square'.format(number))

    def quadrant_middle(self, quadrant):
        q_range = self.quadrants[quadrant]
        possible_middle = int((q_range[0] + q_range[1]) / 2)
        if quadrant in [Quadrant.LEFT, Quadrant.TOP]:
            return possible_middle + 1
        return possible_middle


def first_odd_square_above(num):
    i = 1
    while True:
        next_sq = i * i
        if next_sq >= num:
            return Square(i, next_sq)
        i += 2


def main():
    num = int(sys.argv[1])

    square = first_odd_square_above(num)
    quadrant = square.quadrant(num)
    middle = square.quadrant_middle(quadrant)
    distance_to_middle = abs(num - middle)
    distance_to_center = math.floor(square.root / 2)
    print(distance_to_middle + distance_to_center)


main()
