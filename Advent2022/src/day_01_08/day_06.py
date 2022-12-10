#fmt: off
import sys
sys.path.append(".")

from advent import Advent
#fmt: on


def isMarker(seg):
    return all([l != r for i, l in enumerate(seg[:-1]) for r in seg[i+1:]])


class Day(Advent):
    day = "06"

    def parseState(self, f):
        self.results = []

    def process(self, command, len):
        i = len
        while not isMarker(command[i-len:i]):
            i += 1

        self.results += [i]

    def process1(self, command):
        return self.process(command, 4)

    def result1(self):
        return self.results

    def process2(self, command):
        return self.process(command, 14)


day = Day()

day.test1([7, 5, 6, 10, 11])
day.pass1()

day.test2([19, 23, 23, 29, 26])
day.pass2()
