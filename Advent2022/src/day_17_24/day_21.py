#fmt: off
import sys
import traceback
sys.path.append(".")

from advent import Advent
#fmt: on


class Day(Advent):
    day = "21"


day = Day()

try:
    day.test1(152)
    day.pass1()

    day.test2(None)
    day.pass2()
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
