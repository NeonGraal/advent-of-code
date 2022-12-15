#fmt: off
import re
import sys
import traceback
sys.path.append(".")

from advent import Advent, Point
#fmt: on

breaks = re.compile("[,=:]")
num = re.compile("-?\d+")


class ListSet:
    def __init__(self):
        self.parts = []

    def add(self, begin, end):
        p = self.parts
        if len(p) == 0:
            self.parts = [(begin, end)]
            return
        if begin > p[-1][1]:
            self.parts.append((begin, end))
            return
        b = 0
        while begin > p[b][1]:
            b += 1
        pb = p[b]
        if end < pb[0]:
            # print(f"Insert at {b} ({begin},{end})")
            self.parts.insert(b, (begin, end))
            return
        start = begin if begin < pb[0] else pb[0]
        e = -1
        while end < p[e][0]:
            e -= 1
        pe = p[e]
        finish = end if end > pe[1] else pe[1]

        if e == -1:
            # print(f"Before {b} and ({begin},{end})")
            self.parts = p[:b] + [(start, finish)]
        else:
            # print(f"Before {b}, ({begin},{end}) and after {e}")
            self.parts = p[:b] + [(start, finish)] + p[e+1:]

    def count(self):
        return sum([e-b+1 for b, e in self.parts])

    def __len__(self):
        return len(self.parts)

    def __repr__(self):
        return repr(self.parts)


class Day(Advent):
    day = "15"
    row = 1

    def parseState(self, f):
        self.sensors = []

    def parseCommand(self, line: str):
        parts = breaks.split(line)
        nums = [int(p) for p in parts if num.match(p)]
        s = Point(nums[0], nums[1])
        b = Point(nums[2], nums[3])
        self.sensors.append((s, b, len(s-b)))

    def __str__(self) -> str:
        return "\n".join([f"{s} -> {b} <> {d}" for s, b, d in self.sensors])

    def result1(self):
        result = ListSet()
        beacons = set()

        for s, b, d in self.sensors:
            if b.y == self.row:
                beacons.add(b.x)
            p = Point(s.x, self.row)
            m = d - len(s - p)
            if m >= 0:
                result.add(s.x - m, s.x + m)

        return result.count() - len(beacons)

    def result2(self):
        size = self.row + 1
        results = [ListSet() for _ in range(size)]

        for s, _, d in self.sensors:
            print(".", end='')
            start = -d if s.y > d else -s.y
            stop = d + 1 if s.y + d < size else size - s.y
            for dy in range(start, stop):
                y = s.y + dy
                dx = d - abs(dy)
                min = s.x - dx if s.x > dx else 0
                max = s.x + dx + 1
                if max > size:
                    max = size
                results[y].add(min, max-1)

        print("!")
        for y, r in enumerate(results):
            if len(r) > 1:
                x = r.parts[0][1] + 1
                return x * 4000000 + y


day = Day()

try:
    day.row = 10
    day.test1(26)
    day.row = 2000000
    day.pass1()

    day.row = 20
    day.test2(56000011)
    day.row = 4000000
    day.pass2()
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
