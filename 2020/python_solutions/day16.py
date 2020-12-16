import functools
import itertools
import sys
import time
from collections import defaultdict


def part1():
    with open(sys.argv[1]) as f:
        fields = {}
        your_ticket = []
        nearby_tickets = []

        lines = list(f.readlines())

        for i, line in enumerate(lines):
            line = line.strip()

            if " or " in line:
                parts = line.split(":")
                field_name = parts[0]

                next_parts = parts[1].split("-")

                first_range = [int(next_parts[0]), int(next_parts[1].split()[0])]

                next_part = line.split(" or ")[1]
                second_range = [int(v) for v in next_part.split("-")]
                fields[field_name] = {
                    "first": first_range,
                    "second": second_range,
                }
                continue

            if line == "your ticket:":
                your_ticket = [int(v) for v in lines[i + 1].split(",")]
                continue

            if line == "nearby tickets:":
                for line in lines[i + 1 :]:
                    nearby_tickets.append([int(v) for v in line.split(",")])

    error_rate = 0
    for nearby_ticket in nearby_tickets:
        for value in nearby_ticket:
            valid = False
            for ranges in fields.values():
                if (value >= ranges["first"][0] and value <= ranges["first"][1]) or (
                    value >= ranges["second"][0] and value <= ranges["second"][1]
                ):
                    valid = True
                    break

            if not valid:
                error_rate += value

    return error_rate


def in_range(value, test_range):
    return value >= test_range[0] and value <= test_range[1]


def valid_for_any_range(value, all_ranges):
    for ranges in all_ranges:
        if any(in_range(value, test_range) for test_range in ranges):
            return True
    return False


def valid_for_any_range_single_range(value, ranges):
    return any(in_range(value, test_range) for test_range in ranges)


def field_order_valid(name_order, fields, nearby_ticket):
    for i, (value, ranges) in enumerate(
        zip(nearby_ticket, [fields[f] for f in name_order])
    ):
        if not valid_for_any_range_single_range(value, ranges):
            return False
    return True


def check_order(your_ticket, nearby_tickets, fields, name_order):
    named_fields = []
    names = {}
    for nearby_ticket in nearby_tickets + [your_ticket]:
        if not field_order_valid(name_order, fields, nearby_ticket):
            return False

    return True


def field_valid_for_column(nearby_tickets, column, field, ranges):
    for ticket in nearby_tickets:
        if not valid_for_any_range_single_range(ticket[column], ranges):
            return False
    return True


def part2():
    with open(sys.argv[1]) as f:
        fields = {}
        your_ticket = []
        nearby_tickets = []

        lines = list(f.readlines())

        for i, line in enumerate(lines):
            line = line.strip()

            if " or " in line:
                parts = line.split(":")
                field_name = parts[0]

                next_parts = parts[1].split("-")

                first_range = [int(next_parts[0]), int(next_parts[1].split()[0])]

                next_part = line.split(" or ")[1]
                second_range = [int(v) for v in next_part.split("-")]
                fields[field_name] = [
                    first_range,
                    second_range,
                ]
                continue

            if line == "your ticket:":
                your_ticket = [int(v) for v in lines[i + 1].split(",")]
                continue

            if line == "nearby tickets:":
                for line in lines[i + 1 :]:
                    nearby_tickets.append([int(v) for v in line.split(",")])

    filtered_tickets = []
    for nearby_ticket in nearby_tickets:
        valid = True
        for value in nearby_ticket:
            if not valid_for_any_range(value, list(fields.values())):
                valid = False
                break

        if valid:
            filtered_tickets.append(nearby_ticket)

    field_valid_columns = defaultdict(list)
    for field, ranges in fields.items():
        for i in range(len(your_ticket)):
            if field_valid_for_column(
                filtered_tickets + [your_ticket], i, field, ranges
            ):
                field_valid_columns[field].append(i)

    selected_fields = {}
    while len(selected_fields) != len(fields):
        for field, valid in field_valid_columns.items():
            if len(valid) == 1:
                selected_fields[field] = valid[0]
                field_valid_columns.pop(field, None)
                new_fields = {}
                for other_field, other_valid in field_valid_columns.items():
                    new_fields[other_field] = list(set(other_valid) - {valid[0]})
                field_valid_columns = new_fields
                break

    total = 1
    for field, position in selected_fields.items():
        if field.startswith("departure"):
            total *= your_ticket[position]
    return total


print("Part 1: {}".format(part1()))
print("Part 2: {}".format(part2()))
