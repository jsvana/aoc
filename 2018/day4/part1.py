from datetime import datetime
from typing import List, Tuple


class Guard:
    def __init__(self, id: str) -> None:
        self.id = id
        self.shifts = []
        self.total_time = 0

    def add_shift(self, start: int, end: int) -> None:
        self.shifts.append((start, end))
        self.total_time += end - start

    @property
    def _minutes(self) -> List[int]:
        minutes = [0 for _ in range(60)]
        for start, end in self.shifts:
            for i in range(start, end):
                minutes[i] += 1
        return minutes

    @property
    def most_common_minute(self) -> Tuple[int, int]:
        max_i = 0
        max_count = 0
        for i, count in enumerate(self._minutes):
            if count > max_count:
                max_count = count
                max_i = i

        return max_i, max_count


guards = {}
current_id = None
with open("input.txt") as f:
    lines = []
    for line in f.readlines():
        line = line.strip()
        if not line:
            continue
        lines.append(line)

lines.sort(key=lambda d: datetime.strptime(d[1:17], "%Y-%m-%d %H:%M"))

start = None
end = None

for line in lines:
    parts = line.split()
    if "Guard" in line:
        start = None
        end = None
        current_id = parts[3][1:]
        if current_id not in guards:
            guards[current_id] = Guard(current_id)
        continue

    if current_id is None:
        continue

    if start is None:
        start = int(parts[1][3:5])
        continue

    guards[current_id].add_shift(start, int(parts[1][3:5]))
    start = None
    end = None

max_count = 0
max_minute = 0
max_id = None
for id, guard in guards.items():
    minute, count = guard.most_common_minute
    if count > max_count:
        max_count = count
        max_minute = minute
        max_id = id

print(f"{int(max_id) * max_minute}")
