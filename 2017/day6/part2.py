import operator
import re
import sys


def main():
    with open(sys.argv[1], 'r') as f:
        banks = [int(n) for n in re.split('\s+', f.read().strip())]

    to_count = 1
    if len(sys.argv) == 3:
        to_count = int(sys.argv[2])

    configs = set()
    count = 0
    total_counted = 0
    while True:
        idx, blocks = max(enumerate(banks), key=operator.itemgetter(1))
        banks[idx] = 0
        while blocks > 0:
            idx = (idx + 1) % len(banks)
            banks[idx] += 1
            blocks -= 1

        count += 1
        banks_str = ''.join([str(b) for b in banks])
        if banks_str in configs:
            configs = set()
            total_counted += 1
            print('found after {} rotations'.format(count))
            count = 0
            if total_counted == to_count:
                break

        configs.add(banks_str)


main()
