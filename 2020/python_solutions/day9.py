import sys
from collections import defaultdict
import itertools

window_sum = [0] * 25
out_numbers = []
with open(sys.argv[1]) as f:
    for line in f.readlines():
        line = line.strip()
        out_numbers.append(int(line))

def first_bad_num(numbers, preamble_size):
    for i, number in enumerate(numbers[preamble_size:]):
        found = False
        for number_a, number_b in itertools.combinations(numbers[i:preamble_size + i], 2):
            if number == number_a + number_b:
                found = True
                break

        if not found:
            return number

target = 1639024365
i = 0
while i < len(out_numbers):
    numbers = []
    running_sum = 0
    j = i
    i += 1
    while j < len(out_numbers):
        numbers.append(out_numbers[j])
        running_sum += out_numbers[j]
        j += 1
        if running_sum == target:
            if len(numbers) == 1:
                numbers = []
                continue
            print(min(numbers) + max(numbers))
        elif running_sum > target:
            numbers = []
