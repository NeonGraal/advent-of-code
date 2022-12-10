#fmt: off
import sys
import traceback
sys.path.append(".")

from advent import Advent
#fmt: on


class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __hash__(self):
        return hash(self.x + 999) * 3333 + hash(self.y + 999)

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def __str__(self):
        return f"{self.x},{self.y}"

    def __iadd__(self, other):
        self.x += other.x
        self.y += other.y
        return self

    def __irshift__(self, other):
        (dx, dy) = (other.x - self.x, other.y - self.y)
        (ax, ay) = (abs(dx), abs(dy))
        if ax > 1 or ay > 1:
            if ax > 0:
                self.x += dx // ax
            if ay > 0:
                self.y += dy // ay
        return self


class Day(Advent):
    day = "09"

    def parseState(self, f):
        self.tail = Point(0, 0)
        self.head = Point(0, 0)
        self.rope = [Point(0, 0) for _ in range(10)]
        self.visited = set([self.tail])

    def parseCommand(self, line):
        parts = line.split()
        dist = int(parts[1])

        return (parts[0], dist)

    def process1(self, command):
        for _ in range(command[1]):
            match command[0]:
                case "L":
                    self.head += Point(-1, 0)
                case "R":
                    self.head += Point(1, 0)
                case "U":
                    self.head += Point(0, -1)
                case "D":
                    self.head += Point(0, 1)
            self.tail >>= self.head
            self.visited.add(self.tail)

    def result1(self):
        return len(self.visited)

    def process2(self, command):
        for _ in range(command[1]):
            match command[0]:
                case "L":
                    self.rope[0] += Point(-1, 0)
                case "R":
                    self.rope[0] += Point(1, 0)
                case "U":
                    self.rope[0] += Point(0, -1)
                case "D":
                    self.rope[0] += Point(0, 1)
            for i in range(1, 10):
                self.rope[i] >>= self.rope[i-1]
            self.visited.add(self.rope[9])


day = Day()

try:
    day.test1(13)
    day.pass1()

    day.test2(1)
    day.test2(36, "a")
    day.pass2()
    # 2515 is too low
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
