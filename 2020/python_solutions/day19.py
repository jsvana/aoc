import itertools
import sys
import time
from collections import defaultdict
import functools


class SimpleRule:
    def __init__(self, pattern):
        self.pattern = pattern

    def __repr__(self):
        return " ".join([str(i) for i in self.pattern])

    def check(self, rules, string, start_index):
        if start_index < 0:
            raise ValueError("wat")
        print("checking SimpleRule {} idx {}".format(self, start_index))
        string_index = start_index
        for item in self.pattern:
            if isinstance(item, int):
                ok, new_idx = rules[item].check(rules, string, string_index)
                if not ok:
                    print("  no match")
                    return False, -1
                string_index = new_idx
            else:
                if string_index >= len(string):
                    return False, -1
                    print("uh oh, {} longer than {}. SimpleRule {}, input {}".format(string_index, len(string), self, string))
                if string[string_index] == item:
                    string_index += 1
                else:
                    print("  no match")
                    return False, -1

        print("  matches, idx {}".format(string_index))
        return True, string_index



class OrRule:
    def __init__(self, left_pattern, right_pattern):
        self.left_pattern = left_pattern
        self.right_pattern = right_pattern

    def __repr__(self):
        return "{} | {}".format(self.left_pattern, self.right_pattern)

    def check(self, rules, string, start_index):
        print("checking OrRule {} idx {}".format(self, start_index))
        if start_index < 0:
            raise ValueError("wat")

        string_index = start_index

        ok, new_index = self.left_pattern.check(rules, string, string_index)
        if ok:
            print("  left matches, idx {}".format(new_index))
            return True, new_index

        ok, new_index = self.right_pattern.check(rules, string, string_index)
        if ok:
            print("  right matches, idx {}".format(new_index))
            return True, new_index

        print("  no match")
        return False, -1

def string_matches(rules, string):
    pass

def part1():
    rules = {}
    strings = []
    with open(sys.argv[1]) as f:
        lines = list(f.readlines())
        for i in range(len(lines)):
            line = lines[i]
            i += 1
            line = line.strip()

            if not line:
                strings = [l.strip() for l in lines[i:]]
                break

            i = line.index(":")
            key = int(line[:i])
            rest = line[i + 1:].split()

            left = []
            i = 0
            while i < len(rest) and rest[i] != "|":
                if rest[i].startswith('"'):
                    value = rest[i].replace('"', "")
                else:
                    value = int(rest[i])
                left.append(value)
                i += 1

            i += 1

            right = []
            for value in rest[i:]:
                if value.startswith('"'):
                    value = value.replace('"', "")
                else:
                    value = int(value)
                right.append(value)

            if right:
                rules[key] = OrRule(SimpleRule(left), SimpleRule(right))
            else:
                rules[key] = SimpleRule(left)

    # print(rules)
    # print(strings)

    strings = ["babbbbaabbbbbabbbbbbaabaaabaaa"]
    count = 0
    for string in strings:
        print("Testing {}".format(string))
        ok, index = rules[0].check(rules, string, 0)
        if ok:
            if index == len(string):
                print(string)
                count += 1
            else:
                print("ok but failure, ex {} ac {}".format(len(string), index))
        else:
            print("not ok")

    return count

print("Part 1: {}".format(part1()))
