#fmt: off
import sys
import traceback
sys.path.append(".")

from advent import Advent, Point
#fmt: on


def possibleSort(curr):
    return curr[2] + curr[1]


class Day(Advent):
    day = "12"

    def parseState(self, f):
        self.heights = []
        for y, l in enumerate(f):
            if (pos := l.find("S")) >= 0:
                self.start = Point(pos, y)
                l = l[:pos] + "a" + l[pos+1:]
            if (pos := l.find("E")) >= 0:
                self.end = Point(pos, y)
                l = l[:pos] + "z" + l[pos+1:]
            self.heights.append([ord(c) for c in l if c >= 'a' and c <= 'z'])

    def __str__(self):
        return "\n".join([f"{self.start} -> {self.end} "] + ["".join([chr(c) for c in h]) for h in self.heights])

    def steps(self, curr):
        at = curr[0]
        steps = curr[1] + 1
        r = self.heights[at.y]
        h = r[at.x] - 1

        if (x := at.x - 1) >= 0 and h <= r[x]:
            yield (Point(x, at.y), steps, r[x])
        if (x := at.x + 1) < len(r) and h <= r[x]:
            yield (Point(x, at.y), steps, r[x])
        if (y := at.y - 1) >= 0 and h <= self.heights[y][at.x]:
            yield (Point(at.x, y), steps, self.heights[y][at.x])
        if (y := at.y + 1) < len(self.heights) and h <= self.heights[y][at.x]:
            yield (Point(at.x, y), steps, self.heights[y][at.x])

    def result1(self):
        tried = set([self.end])
        possible = [(self.end, 0, ord('z'))]

        while curr := possible.pop(0):
            if curr[0] == self.start:
                return curr[1]

            for n in self.steps(curr):
                if n[0] not in tried:
                    tried.add(n[0])
                    possible += [n]
            possible.sort(key=possibleSort)

        return -1

    def result2(self):
        tried = set([self.end])
        possible = [(self.end, 0, ord('z'))]

        while curr := possible.pop(0):
            if curr[2] == ord('a'):
                return curr[1]

            for n in self.steps(curr):
                if n[0] not in tried:
                    tried.add(n[0])
                    possible += [n]
            possible.sort(key=possibleSort)

        return -1


day = Day()

try:
    day.test1(31)
    day.pass1()

    day.test2(29)
    day.pass2()
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
