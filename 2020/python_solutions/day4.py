def valid_byr(s):
    if len(s) != 4:
        return False

    s = int(s)
    return s >= 1920 and s <= 2002

def valid_iyr(s):
    if len(s) != 4:
        return False

    s = int(s)
    return s >= 2010 and s <= 2020

def valid_eyr(s):
    if len(s) != 4:
        return False

    s = int(s)
    return s >= 2020 and s <= 2030

def valid_hgt(s):
    if s.endswith("cm"):
        s = int(s[:-2])
        return s >= 150 and s <= 193
    elif s.endswith("in"):
        s = int(s[:-2])
        return s >= 59 and s <= 76
    else:
        return False

def valid_hcl(s):
    if len(s) != 7:
        return False

    if s[0] != "#":
        return False

    for c in s[1:]:
        if c not in {"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f"}:
            return False

    return True

def valid_ecl(s):
    return s in {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}

def valid_pid(s):
    if len(s) != 9:
        return False

    for c in s:
        if c not in {"0", "1", "2", "3", "4", "5", "6", "7", "8", "9"}:
            return False

    return True

validators = {
    "byr": valid_byr,
    "iyr": valid_iyr,
    "eyr": valid_eyr,
    "hgt": valid_hgt,
    "hcl": valid_hcl,
    "ecl": valid_ecl,
    "pid": valid_pid,
    "cid": lambda s: True,
}

with open("inputs/day4/input.txt") as f:
    required_fields = {
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
    }

    right = 0
    fields = {}
    for line in f.readlines():
        if not line.strip():
            fields.pop("cid", None)
            if set(fields) == required_fields:
                right += 1

            fields = {}
            continue

        for token in line.split():
            parts = token.split(":")

            valid = validators[parts[0]](parts[1])
            if valid:
                fields[parts[0]] = parts[1]

    print(right)
