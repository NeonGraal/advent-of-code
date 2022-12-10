#fmt: off
import sys
sys.path.append(".")

from advent import Advent
#fmt: on


def pathJoin(dir: list):
    return "/".join([""] + dir) if dir else "/"


class Day(Advent):
    day = "07"

    def parseState(self, f):
        self.dirs = {"/": 0}
        self.cwd = []

    def parseCommand(self, line):
        parts = line.split()
        if parts[0] == "$":
            if parts[1] != "cd":
                return
            if parts[2] == "/":
                self.cwd = []
            elif parts[2] == "..":
                self.cwd.pop()
            else:
                self.cwd += [parts[2]]
            return

        if parts[0] == "dir":
            path = pathJoin(self.cwd + [parts[1]])
            self.dirs[path] = 0
        else:
            size = int(parts[0])
            for l in range(len(self.cwd)+1):
                path = pathJoin(self.cwd[:l])
                self.dirs[path] += size

    def result1(self):
        return sum(self.dirs[k] for k in self.dirs if self.dirs[k] < 100000)

    def result2(self):
        unused = 70000000 - self.dirs["/"]
        needed = 30000000

        return min([self.dirs[k] for k in self.dirs if self.dirs[k] + unused > needed])


day = Day()

try:
    day.test1(95437)
    day.pass1()

    day.test2(24933642)
    day.pass2()
except:
    (_, ex, tb) = sys.exc_info()
    print(day.dirs)
    print(f"EXCEPTION: {ex} {tb}")
