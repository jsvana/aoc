import itertools
import sys
import time
from collections import defaultdict
import functools

def evaluate(expression):
    # print("EVALUATING {}".format(expression))
    running_value = None
    current_op = None
    current_left = None

    while expression:
        # print("LOOP {}".format(expression))
        token = expression[0]
        expression = expression[1:]
        # print("Token: {}, expression: {}".format(token, expression))

        if token == "(":
            # print("recurse")
            expression = evaluate(expression)
            if not isinstance(expression, list):
                expression = [expression]
        elif token == ")":
            if expression:
                return [current_left] + expression
            return [current_left]
        elif token in {"*", "+"}:
            # print("consuming op")
            if current_left is None:
                raise ValueError("No left operand for expression")

            current_op = token
        else:
            # print("number")
            if current_left is None:
                # print("no current_left")
                current_left = int(token)
            else:
                # print("operator")
                if current_op is None:
                    raise ValueError("No operator for expression but left operand assigned")

                if current_op == "+":
                    result = current_left + int(token)
                elif current_op == "*":
                    result = current_left * int(token)
                else:
                    raise ValueError("Unknown operator {}".format(token))

                current_left = None
                current_op = None

                # print("Pre assign: {}".format(expression))
                expression = [result] + expression

    return current_left


def part1():
    expressions = []
    with open(sys.argv[1]) as f:
        for line in f.readlines():
            line = line.strip()
            expression = line.split()
            result = []
            for token in expression:
                while token.startswith("("):
                    result.append("(")
                    token = token[1:]

                to_append = []
                while token.endswith(")"):
                    to_append.append(")")
                    token = token[:-1]

                result.append(token)

                result.extend(to_append)

            expressions.append(result)

    total = 0
    for expression in expressions:
        result = evaluate(expression)
        total += result

    return total

def add_parens(expression):
    i = 0
    while i < len(expression):
        token = expression[i]
        if token == "+":
            # print("  before + at {}: {}".format(i, " ".join(expression)))
            # print("  before open paren: {}".format(" ".join(expression)))
            paren_count = 0
            j = 1
            while expression[i - j] == ")":
                # print("    adding paren")
                paren_count += 1
                j += 1

            """
            try:
                int(expression[i - j])
                j += 1
            except ValueError:
                pass
            """

            while paren_count > 0:
                # print("    moving backward")
                j += 1
                if expression[i - j] == "(":
                    # print("    removing paren")
                    paren_count -= 1

            expression = expression[:i - j] + ["("] + expression[i - j:]
            # print("  after open paren: {}".format(" ".join(expression)))

            # print("starting close paren")
            i += 1

            paren_count = 0
            j = 1
            # print("  i: {}, i + j: {}, expression[i + j]: {}".format(i, i + j, expression[i + j]))
            while expression[i + j] == "(":
                # print("    adding paren")
                paren_count += 1
                j += 1

            """
            try:
                int(expression[i + j])
                j += 1
            except ValueError:
                pass
            """

            while paren_count > 0:
                j += 1
                # print("    advancing")
                if expression[i + j] == ")":
                    # print("    removing paren")
                    paren_count -= 1

            expression = expression[:i + j + 1] + [")"] + expression[i + j + 1:]

            # print("  after + at {}: {}".format(i, " ".join(expression)))
        i += 1

    return expression

def part2():
    expressions = []
    with open(sys.argv[1]) as f:
        for line in f.readlines():
            line = line.strip()
            expression = line.split()
            result = []
            for token in expression:
                while token.startswith("("):
                    result.append("(")
                    token = token[1:]

                to_append = []
                while token.endswith(")"):
                    to_append.append(")")
                    token = token[:-1]

                result.append(token)

                result.extend(to_append)

            expressions.append(result)

    # Add parens
    new_expressions = []
    for expression in expressions:
        print()
        print("Before: {}".format(" ".join(expression)))
        expression = add_parens(expression)
        print("After: {}".format(" ".join(expression)))
        print()
        new_expressions.append(expression)

    total = 0
    for expression in new_expressions:
        result = evaluate(expression)
        total += result

    return total

print("Part 1: {}".format(part1()))
print("Part 2: {}".format(part2()))
