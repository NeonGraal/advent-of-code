#fmt: off
import sys
import traceback
sys.path.append(".")

from advent import Advent, Point
#fmt: on


class Day(Advent):
    day = "14"

    def parseState(self, f):
        self.cave = []
        self.min_x = 495
        self.max_x = 505

    def check_cave(self, y, sx, ex):
        if y >= len(self.cave):
            for i in range(len(self.cave), y + 1):
                self.cave.append("." * 999)
        if sx - 5 < self.min_x:
            self.min_x = sx - 5
        if ex + 5 > self.max_x:
            self.max_x = ex + 5

    def draw_x(self, y, sx, ex):
        if ex < sx:
            (sx, ex) = (ex, sx)
        self.check_cave(y, sx, ex)
        row = self.cave[y]
        self.cave[y] = row[:sx] + "#" * (ex - sx + 1) + row[ex+1:]

    def draw_y(self, x, sy, ey):
        if ey < sy:
            (sy, ey) = (ey, sy)
        self.check_cave(ey, x, x)
        for y in range(sy, ey+1):
            row = self.cave[y]
            self.cave[y] = row[:x] + "#" + row[x+1:]

    def parseCommand(self, line: str):
        points = [Point.parse(p) for p in line.split(" -> ")]

        start = points.pop(0)
        for e in points:
            if start.y == e.y:
                self.draw_x(e.y, start.x, e.x)
            else:
                self.draw_y(e.x, start.y, e.y)
            start = e

    def fall(self, at):
        if at.y >= len(self.cave) - 1:
            at += Point(0, 1)
            return True
        row = self.cave[at.y+1]
        if row[at.x] == ".":
            at += Point(0, 1)
            return True
        if row[at.x-1] == ".":
            at += Point(-1, 1)
            return True
        if row[at.x+1] == ".":
            at += Point(1, 1)
            return True

        row = self.cave[at.y]
        self.cave[at.y] = row[:at.x] + "o" + row[at.x+1:]
        return False

    def result1(self):
        count = 0
        while True:
            grain = Point(500, 0)

            while self.fall(grain):
                if grain.y >= len(self.cave):
                    print(self)
                    return count

            count += 1

    def result2(self):
        self.cave += ["." * 999, "#" * 999]
        
        count = 0
        while True:
            grain = Point(500, 0)

            while self.fall(grain):
                pass
            
            count += 1
            if grain.y == 0:
                return count

    def __str__(self):
        return "\n".join([f"{self.min_x} - {self.max_x} , {len(self.cave)}"] + [r[self.min_x:self.max_x] for r in self.cave])


day = Day()

try:
    day.test1(24)
    day.pass1()

    day.test2(93)
    day.pass2()
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
