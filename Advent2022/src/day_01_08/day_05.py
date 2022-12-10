#fmt: off
import sys
sys.path.append(".")

from advent import Advent
#fmt: on


class Day(Advent):
    day = "05"
    stacks = [""] * 9

    def parseState(self, f):
        self.stacks = [""] * 9
        while (line := f.readline())[1] != "1":
            crates = line[1::4]
            for i, c in enumerate(crates):
                if c > " ":
                    self.stacks[i] += c

    def __str__(self):
        return "\n".join([f"{i}: {s}" for i, s in enumerate(self.stacks) if s > ""])

    def parseCommand(self, command):
        parts = command.split()
        if len(parts) < 1 or parts[0] != "move":
            return (0, 0, 0)
        (cnt, frm, to) = [int(p) for p in parts[1::2]]
        frm -= 1
        to -= 1

        return (cnt, frm, to)

    def process1(self, params):
        (cnt, frm, to) = params
        for i in range(cnt):
            if self.stacks[frm] > "":
                self.stacks[to] = self.stacks[frm][0] + self.stacks[to]
                self.stacks[frm] = self.stacks[frm][1:]

    def result1(self):
        return "".join([s[:1] for s in self.stacks])

    def process2(self, params):
        (cnt, frm, to) = params
        self.stacks[to] = self.stacks[frm][:cnt] + self.stacks[to]
        self.stacks[frm] = self.stacks[frm][cnt:]


day = Day()

day.test1("CMZ")
day.pass1()

day.test2("MCD")
day.pass2()
