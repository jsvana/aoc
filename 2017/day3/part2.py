from itertools import product
import sys


def sequence():
    sequence_map = {(0, 0): 1}
    radius = 1
    x = 0
    y = 0
    d_x = 1
    d_y = 0
    while True:
        x += d_x
        y += d_y
        if x == radius and y == radius - 1:
            d_x = 0
            d_y = -1
        elif x == radius and y == -radius:
            d_x = -1
            d_y = 0
        elif x == -radius and y == -radius:
            d_x = 0
            d_y = 1
        elif x == -radius and y == radius:
            d_x = 1
            d_y = 0
        elif x == radius and y == radius:
            radius += 1

        val = 0
        for p_x, p_y in product(range(3), range(3)):
            if p_x == 1 and p_y == 1:
                continue

            val += sequence_map.get((x + p_x - 1, y + p_y - 1), 0)
        sequence_map[(x, y)] = val
        yield val


def main():
    num = int(sys.argv[1])

    seq = sequence()

    while True:
        seq_next = next(seq)
        if seq_next > num:
            print(seq_next)
            break


main()
