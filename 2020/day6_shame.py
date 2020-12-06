import sys
from collections import defaultdict

groups = []
current_group = {
    "members": 0,
    "counts": defaultdict(int),
}

with open(sys.argv[1]) as f:
    for line in f.readlines():
        line = line.strip()
        if not line:
            groups.append(current_group)
            current_group = {
                "members": 0,
                "counts": defaultdict(int),
            }
            continue

        current_group["members"] += 1
        for c in line:
            current_group["counts"][c] += 1

groups.append(current_group)

total = 0
for group in groups:
    group_count = 0
    for count in group["counts"].values():
        if count == group["members"]:
            group_count += 1

    total += group_count

print(total)
