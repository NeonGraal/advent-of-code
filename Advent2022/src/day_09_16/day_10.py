#fmt: off
import sys
import traceback
sys.path.append(".")

from advent import Advent
#fmt: on


class Day(Advent):
    day = "10"

    def parseState(self, f):
        self.x = 1
        self.cycle = 0

        self.sum = 0
        self.captures = set([20, 60, 100, 140, 180, 220])
        self.screen = []
        self.line = ""

    def parseCommand(self, line):
        parts = line.split()

        return (parts[0], int(parts[1]) if len(parts) > 1 else 0)

    def step1(self):
        self.cycle += 1
        if self.cycle in self.captures:
            capture = self.x * self.cycle
            self.sum += capture

    def process1(self, command):
        if command[0] == "noop":
            self.step1()
            return
        if command[0] != "addx":
            return

        self.step1()
        self.step1()
        self.x += command[1]

    def result1(self):
        return self.sum

    def step2(self):
        pos = self.cycle % 40
        self.cycle += 1
        self.line += "#" if abs(pos - self.x) < 2 else " "
        if pos == 39:
            self.screen += [self.line]
            self.line = ""

    def process2(self, command):
        if command[0] == "noop":
            self.step2()
            return
        if command[0] != "addx":
            return

        self.step2()
        self.step2()
        self.x += command[1]

    def result2(self):
        for s in self.screen:
            print(s)


day = Day()

try:
    day.test1(13140)
    day.pass1()

    day.test2(None)
    day.pass2()
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
