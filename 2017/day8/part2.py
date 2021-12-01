from collections import defaultdict
import re
import sys


class Condition:

    OPS = {
        '<': lambda a, b: a < b,
        '>': lambda a, b: a > b,
        '<=': lambda a, b: a <= b,
        '>=': lambda a, b: a >= b,
        '==': lambda a, b: a == b,
        '!=': lambda a, b: a != b,
    }

    def __init__(self, register, op, value):
        self.register = register
        self.op = op
        self.value = value

    def eval(self, registers):
        return self.OPS[self.op](registers[self.register], self.value)


class Instruction:

    def __init__(self, line):
        m = re.match(
            '([a-z]+) (inc|dec) (-?\d+) if ([a-z]+) ([<>=!]+) (-?\d+)',
            line,
        )
        self.register = m.group(1)
        self.op = m.group(2)
        self.magnitude = int(m.group(3))
        self.condition = Condition(m.group(4), m.group(5), int(m.group(6)))

    def eval(self, registers):
        if not self.condition.eval(registers):
            return

        if self.op == 'inc':
            registers[self.register] += self.magnitude
        else:
            registers[self.register] -= self.magnitude


def main():
    with open(sys.argv[1], 'r') as f:
        instructions = [Instruction(l.strip()) for l in f.readlines()]

    registers = defaultdict(int)
    maxes = []
    for instruction in instructions:
        instruction.eval(registers)
        maxes.append(max(registers.values()))

    print(max(maxes))


main()
