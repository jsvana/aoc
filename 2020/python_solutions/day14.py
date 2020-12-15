import itertools
import sys
import time
from collections import defaultdict
import functools

def part1():
    memory = {}

    with open(sys.argv[1]) as f:
        mask = None
        for line in f.readlines():
            line = line.strip()
            if line.startswith("mask = "):
                mask = line.split(" = ")[1]
            else:
                address = int(line.split("[")[1].split("]")[0])
                value = int(line.split(" = ")[1])
                value = "{:036b}".format(value)
                val = []
                for i, c in enumerate(value):
                    if mask[i] == "X":
                        val.append(c)
                    else:
                        val.append(mask[i])

                memory[address] = int("".join(val), 2)

    s = 0
    for v in memory.values():
        s += v

    return s


def part2():
    memory = {}

    with open(sys.argv[1]) as f:
        mask = None
        for line in f.readlines():
            line = line.strip()
            if line.startswith("mask = "):
                mask = line.split(" = ")[1]
            else:
                input_address = int(line.split("[")[1].split("]")[0])
                input_value = int(line.split(" = ")[1])
                address_binary = "{:036b}".format(input_address)

                addresses = [[]]
                for i, c in enumerate(address_binary):
                    if mask[i] == "X":
                        new_addresses = []
                        for address in addresses:
                            new_addresses.append(address + ["0"])
                            new_addresses.append(address + ["1"])
                        addresses = new_addresses
                    elif mask[i] == "1":
                        new_addresses = []
                        for address in addresses:
                            address.append("1")
                            new_addresses.append(address)
                        addresses = new_addresses
                    else:
                        new_addresses = []
                        for address in addresses:
                            address.append(c)
                            new_addresses.append(address)
                        addresses = new_addresses

                for address in addresses:
                    memory[int("".join(address))] = input_value

    s = 0
    for v in memory.values():
        s += v

    return s

print("Part 1: {}".format(part1()))
print("Part 2: {}".format(part2()))
