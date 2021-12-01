import copy
import re
from typing import Dict, List, Optional, Tuple


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

    @property
    def area(self) -> int:
        return self.w * self.h


FieldType = Dict[Tuple[int, int], str]


class Field:
    def __init__(self):
        self.field: FieldType = {}
        self.extents = 0, 0
        self.rects: List[Rect] = []

    def add_rect(self, rect: Rect) -> None:
        self.rects.append(rect)

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

    def _fill_rect(self, rect: Rect, field: FieldType) -> int:
        filled_area = 0
        for i in range(rect.h):
            for j in range(rect.w):
                key = rect.x + j, rect.y + i
                if field.get(key) == rect.id:
                    filled_area += 1
                field[key] = MULTIPLE
        return filled_area

    def check_rects(self) -> Optional[Rect]:
        field = copy.copy(self.field)
        for rect in self.rects:
            filled_area = self._fill_rect(rect, field)
            if filled_area == rect.area:
                return rect.id
        return None


field = Field()

with open('input.txt') as f:
    for line in f.readlines():
        line = line.strip()
        if not line:
            continue
        field.add_rect(Rect.from_str(line))

print(field)
print(f"Multiple claims: {field.multiple_claim_count}")
print(field.check_rects())
