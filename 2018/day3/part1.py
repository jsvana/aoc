import re
from typing import Dict, Tuple


MULTIPLE = 'X'


class Rect:
    def __init__(self, id: str, x: int, y: int, w: int, h: int) -> None:
        self.id = id
        self.x = x
        self.y = y
        self.w = w
        self.h = h

    @classmethod
    def from_str(cls, line: str) -> "Rect":
        m = re.match(r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)', line)
        if m is None:
            raise ValueError("wat")
        parts = m.groups()
        return cls(parts[0], *[int(p) for p in parts[1:]])


class Field:
    def __init__(self):
        self.field: Dict[Tuple[int, int], str] = {}
        self.extents = 0, 0

    def add_rect(self, rect: Rect) -> None:
        for i in range(rect.h):
            for j in range(rect.w):
                key = rect.x + j, rect.y + i
                if key in self.field:
                    self.field[key] = MULTIPLE
                else:
                    self.field[key] = rect.id

        self.extents = (
            max(rect.x + rect.w, self.extents[0]),
            max(rect.y + rect.h, self.extents[1]),
        )

    @property
    def multiple_claim_count(self):
        count = 0
        for i in range(self.extents[1]):
            for j in range(self.extents[0]):
                if self.field.get((j, i)) == MULTIPLE:
                    count += 1
        return count

    def __str__(self) -> str:
        lines = []
        for i in range(self.extents[1]):
            line = ""
            for j in range(self.extents[0]):
                line += str(self.field.get((j, i), "."))
            lines.append(line)
        return "\n".join(lines)


field = Field()

with open('input.txt') as f:
    for line in f.readlines():
        line = line.strip()
        if not line:
            continue
        field.add_rect(Rect.from_str(line))

print(field)
print(f"Multiple claims: {field.multiple_claim_count}")
