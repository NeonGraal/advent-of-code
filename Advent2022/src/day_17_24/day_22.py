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

    def cube_row(self, d, next, size):
        if next.x < self.min:
            if next.y < size:  # 1 -> 3
                return "v", Point(size + next.y, size)
            elif next.y < size * 2:  # 2 -> 6
                return "^", Point(size * 5 - next.y - 1, size * 3 - 1)
            else:  # 5 -> 3
                return "^", Point(size * 4 - next.y - 1, size * 2 - 1)
        elif next.x > self.max:
            if next.y < size:  # 1 -> 6
                return "<", Point(size * 4 - 1, size * 3 - next.y - 1)
            elif next.y < size * 2:  # 4 -> 6
                return "v", Point(size * 5 - next.y - 1, size * 2)
            else:  # 6 -> 1
                return "<", Point(size * 3 - 1, size * 3 - next.y - 1)
        return d, next

    def cube_col(self, d, next, size):
        if next.y < self.min:
            if next.x < size:  # 2 -> 1
                return "v", Point(size * 3 - next.x - 1, 0)
            elif next.x < size * 2:  # 3 -> 1
                return ">", Point(size * 2, next.x - size)
            elif next.x < size * 3:  # 1 -> 2
                return "v", Point(size * 3 - next.x - 1, size * 1)
            else:  # 6 -> 4
                return "<", Point(size * 3 - 1, size * 5 - next.x - 1)
        elif next.y > self.max:
            if next.x < size:  # 2 -> 5
                return "^", Point(size * 3 - next.x - 1, size * 3 - 1)
            elif next.x < size * 2:  # 3 -> 5
                return ">", Point(size * 2, size * 3 - next.x - 1)
            elif next.x < size * 3:  # 5 -> 2
                return "^", Point(size * 3 - next.x - 1, size * 2 - 1)
            else:  # 6 -> 2
                return ">", Point(0, size * 5 - next.x - 1)
        return d, next


class Day(Advent):
    day = "22"

    def parseState(self, f):
        self.rows = {}
        self.cols = {}
        self.board = []

        y = 0
        for line in f:
            l = line.rstrip()
            if len(l) < 1:
                return
            self.board.append(l)
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

        self.set(curr, d)
        curr += Point(1, 1)
        print("\n".join(self.board + [f"{curr} - {d}"]))

        return curr.y * 1000 + curr.x * 4 + dirs.find(d)

    def rotate(self, d, cmd):
        curr = dirs.find(d)
        curr += -1 if cmd == "L" else 1
        return dirs[curr % 4]

    def set(self, curr, d):
        row = self.board[curr.y]
        self.board[curr.y] = row[:curr.x] + d + row[curr.x+1:]
        
    def move1(self, curr, d, count):
        by = moves[d]
        if by.y == 0:
            row = self.rows[curr.y]
            while count > 0:
                count -= 1
                self.set(curr, d)
                next = row.move(curr.x, by.x)
                if next == curr.x:
                    break
                curr = Point(next, curr.y)
            return curr

        col = self.cols[curr.x]
        while count > 0:
            count -= 1
            self.set(curr, d)
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
                (d, curr) = self.move2(curr, d, int(cmd))

        self.set(curr, d)
        curr += Point(1, 1)
        print("\n".join(self.board + [f"{curr} - {d}"]))

        return curr.y * 1000 + curr.x * 4 + dirs.find(d)

    def move2(self, curr, d, count):
        size = len(self.rows) // 3
        while count > 0:
            count -= 1
            self.set(curr, d)
            by = moves[d]
            if by.y == 0:
                (n, next) = self.rows[curr.y].cube_row(d, curr + by, size)
                if next.x >= size*4:
                    raise Exception(f"{curr} {d}({by}) -> {next} {n}")
                    
                row = self.rows[next.y]
                if next.x in row.walls:
                    break
                (d, curr) = (n, next)
            else:
                if curr.x >= size*4:
                    raise Exception(f"{curr} {d}({by})")
                (n, next) = self.cols[curr.x].cube_col(d, curr + by, size)
                if next.x >= size*4:
                    raise Exception(f"{curr} {d}({by}) -> {next} {n}")
                col = self.cols[next.x]
                if next.y in col.walls:
                    break
                (d, curr) = (n, next)

        return d, curr


row_tests = [
    ("1->3", "<", Point(7, 1), ("v", Point(5, 4))),
    ("2->6", "<", Point(-1, 5), ("^", Point(14, 11))),
    ("5->3", "<", Point(7, 9), ("^", Point(6, 7))),
    ("1->6", ">", Point(12, 1), ("<", Point(15, 10))),
    ("4->6", ">", Point(12, 5), ("v", Point(14, 8))),
    ("6->1", ">", Point(16, 9), ("<", Point(11, 2))),
]

col_tests = [
    ("2->1", "^", Point(1, 3), ("v", Point(10, 0))),
    ("3->1", "^", Point(5, 3), (">", Point(8, 1))),
    ("1->2", "^", Point(9, -1), ("v", Point(2, 4))),
    ("6->4", "^", Point(13, 7), ("<", Point(11, 6))),
    ("2->5", "v", Point(1, 8), ("^", Point(10, 11))),
    ("3->5", "v", Point(5, 8), (">", Point(8, 6))),
    ("5->2", "v", Point(9, 12), ("^", Point(2, 7))),
    ("6->2", "v", Point(13, 12), (">", Point(0, 6))),
]

day = Day()

try:
    day.test1(6032)
    day.test1(6032, "a")

    for label, dir, next, expect in row_tests:
        result = day.rows[next.y].cube_row(dir, next, 4)
        assert result == expect, f"{label}: r{result} != e{expect}"
    for label, dir, next, expect in col_tests:
        result = day.cols[next.x].cube_col(dir, next, 4)
        assert result == expect, f"{label}: r{result} != e{expect}"

    day.pass1()

    day.test2(5031)
    day.pass2()
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
