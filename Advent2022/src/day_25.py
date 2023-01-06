#fmt: off
import sys
import traceback
sys.path.append(".")

from advent import Advent
#fmt: on


def fromSnafu(input: str) -> int:
    num = 0
    for c in input:
        num *= 5
        match c:
            case '2': num += 2
            case '1': num += 1
            case '0': num += 0
            case '-': num += -1
            case '=': num += -2
            case _: raise Exception(f"Invalid SNAFU digit {c}")
    return num


def toSnafu(input: int) -> str:

    result = ""
    while True:
        r = input % 5
        inc = 0
        match r:
            case 2: result = "2" + result
            case 1: result = "1" + result
            case 0: result = "0" + result
            case 4:
                result = "-" + result
                inc = 1
            case 3:
                result = "=" + result
                inc = 1
            case _: raise Exception("Invalid SNAFU remainder {r}")
        if abs(input) < 3:
            return result
        input //= 5
        input += inc


class Day(Advent):
    day = "25"

    def parseState(self, f):
        self.sum = 0

    def parseCommand(self, line: str):
        num = fromSnafu(line)
        # print(f"{line} -> {num} ({toSnafu(num)})")
        return num

    def process1(self, command):
        self.sum += command

    def result1(self):
        return toSnafu(self.sum)

    def result2(self):
        pass


day = Day()

try:
    day.test1("2=-1=0")
    day.pass1()

    day.test2(None)
    day.pass2()
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
