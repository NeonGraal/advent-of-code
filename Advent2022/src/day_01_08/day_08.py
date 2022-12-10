#fmt: off
import sys
import traceback
sys.path.append(".")

from advent import Advent
#fmt: on


class Day(Advent):
    day = "08"

    def parseState(self, f):
        self.forest = [l.rstrip() for l in f]
        self.last = len(self.forest) - 1

    def __str__(self):
        return "\n".join(self.forest)

    def isVisible(self, x, y):
        if y == 0 or y == self.last or x == 0 or x == self.last:
            return True
        row = self.forest[y]
        height = row[x]
        return (self.row_tallest(height, row, x, -1) or
                self.row_tallest(height, row, x, 1) or
                self.col_tallest(height, x, y, -1) or
                self.col_tallest(height, x, y, 1))

    def row_tallest(self, h, r, x, dx):
        nx = x + dx
        if nx < 0 or nx > self.last:
            return True
        return r[nx] < h and self.row_tallest(h, r, nx, dx)

    def col_tallest(self, h, x, y, dy):
        ny = y + dy
        if ny < 0 or ny > self.last:
            return True
        return self.forest[ny][x] < h and self.col_tallest(h, x, ny, dy)

    def result1(self):
        size = self.last + 1
        visible = [(x, y) for y in range(size)
                   for x in range(size) if self.isVisible(x, y)]
        return len(visible)

    def scenic(self, x, y):
        row = self.forest[y]
        height = row[x]
        return (self.row_view(height, row, x, -1, 1) *
                self.row_view(height, row, x, 1, 1) *
                self.col_view(height, x, y, -1, 1) *
                self.col_view(height, x, y, 1, 1))

    def row_view(self, h, r, x, dx, c):
        nx = x + dx
        if nx == 0 or nx == self.last or r[nx] >= h:
            return c
        return self.row_view(h, r, nx, dx, c + 1)

    def col_view(self, h, x, y, dy, c):
        ny = y + dy
        if ny == 0 or ny == self.last or self.forest[ny][x] >= h:
            return c
        return self.col_view(h, x, ny, dy, c + 1)

    def result2(self):
        scores = [self.scenic(x, y) for y in range(1, self.last)
                  for x in range(1, self.last)]
        return max(scores)


day = Day()

try:
    day.test1(21)
    day.pass1()

    day.test2(8)
    day.pass2()
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
