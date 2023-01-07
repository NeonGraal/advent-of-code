#fmt: off
import re
import sys
import traceback
sys.path.append(".")

from advent import Advent, Point
#fmt: on

leftRight = re.compile('(L|R)')
cells = re.compile('[.#]+')

dirs = ">v<^"

moves = {">": Point(1, 0),
         "v": Point(0, 1),
         "<": Point(-1, 0),
         "^": Point(0, -1)}


class RowCol:
    def __init__(self, min, max, walls):
        self.min = min
        self.max = max
        self.walls = set(walls)

    def __repr__(self):
        walls = ",".join([str(w) for w in sorted(self.walls)])
        return f"{self.min}-{self.max} : [{walls}]"

    def move(self, start, by):
        next = start + by
        if next > self.max:
            next = self.min
        if next < self.min:
            next = self.max
        return start if next in self.walls else next


class Day(Advent):
    day = "22"
    board = []
    commands = []

    def parseState(self, f):
        self.rows = {}
        self.cols = {}

        y = 0
        for line in f:
            l = line.rstrip()
            if len(l) < 1:
                return
            match = cells.search(l)
            walls = [x for x, c in enumerate(l) if c == '#']
            self.rows[y] = RowCol(match.start(), match.end()-1, walls)

            for x in range(*match.span()):
                if x not in self.cols:
                    self.cols[x] = RowCol(y, y, [])
                col = self.cols[x]
                col.max = y
                if x in walls:
                    col.walls.add(y)
            y += 1

    def parseCommand(self, line: str):
        self.commands = leftRight.split(line)

    def __str__(self) -> str:
        render = [f"{i}- {r}" for i, r in sorted(self.rows.items())]
        render += [f"{i}| {c}" for i, c in sorted(self.cols.items())]
        render += ["===", " ".join(self.commands)]
        return "\n".join(render)

    def result1(self):
        curr = Point(self.rows[0].min, 0)
        d = ">"

        for cmd in self.commands:
            if cmd in ["L", "R"]:
                d = self.rotate(d, cmd)
            else:
                curr = self.move1(curr, d, int(cmd))

        curr += Point(1, 1)

        return curr.y * 1000 + curr.x * 4 + dirs.find(d)

    def rotate(self, d, cmd):
        curr = dirs.find(d)
        curr += -1 if cmd == "L" else 1
        return dirs[curr % 4]

    def move1(self, curr, d, count):
        by = moves[d]
        if by.y == 0:
            row = self.rows[curr.y]
            while count > 0:
                count -= 1
                next = row.move(curr.x, by.x)
                if next == curr.x:
                    break
                curr = Point(next, curr.y)
            return curr

        col = self.cols[curr.x]
        while count > 0:
            count -= 1
            next = col.move(curr.y, by.y)
            if next == curr.y:
                break
            curr = Point(curr.x, next)
        return curr

    def result2(self):
        curr = Point(self.rows[0].min, 0)
        d = ">"

        for cmd in self.commands:
            if cmd in ["L", "R"]:
                d = self.rotate(d, cmd)
            else:
                curr = self.move2(curr, d, int(cmd))

        curr += Point(1, 1)

        return curr.y * 1000 + curr.x * 4 + dirs.find(d)

    def move2(self, curr, d, count):
        while count > 0:
            count -= 1
            by = moves[d]
            if by.y == 0:
                row = self.rows[curr.y]
                next = row.move(curr.x, by.x)
                if next == curr.x:
                    break
                curr = Point(next, curr.y)
            else:
                col = self.cols[curr.x]
                next = col.move(curr.y, by.y)
                if next == curr.y:
                    break
                curr = Point(curr.x, next)
        return curr


day = Day()

try:
    day.test1(6032)
    day.pass1()

    day.test2(5031)
    day.pass2()
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
