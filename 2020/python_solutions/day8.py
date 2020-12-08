import sys
from collections import defaultdict

instructions = []
with open(sys.argv[1]) as f:
    for line in f.readlines():
        line = line.strip()
        parts = line.split()
        instructions.append((parts[0], int(parts[1])))

seen_pcs = set()

running = True


def retval(program):
    seen_pcs = set()

    pc = 0
    accumulator = 0
    while pc >= 0 and pc < len(program):
        if pc in seen_pcs:
            return None

        seen_pcs.add(pc)

        instruction = program[pc]
        if instruction[0] == "acc":
            accumulator += instruction[1]
            pc += 1
        elif instruction[0] == "jmp":
            pc += instruction[1]
        elif instruction[0] == "nop":
            pc += 1

    return accumulator

for i, (instruction, val) in enumerate(instructions):
    should_run = False
    if instruction == "nop":
        new_instructions = instructions[:i] + [("jmp", val)] + instructions[i+1:]
        should_run = True
    elif instruction == "jmp":
        new_instructions = instructions[:i] + [("nop", val)] + instructions[i+1:]
        should_run = True

    if should_run:
        ret = retval(new_instructions)
        if ret:
            print(ret)
