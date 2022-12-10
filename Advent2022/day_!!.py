#fmt: off
import sys
sys.path.append(".")

from advent import Advent
#fmt: on


class Day(Advent):
    day = "!!"


day = Day()

try:
    day.test1(None)
    day.pass1()

    day.test2(None)
    day.pass2()
except:
    (_, ex, tb) = sys.exc_info()
    print(day)
    print(f"EXCEPTION: {ex} {tb}")
