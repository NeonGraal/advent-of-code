#fmt: off
from functools import cmp_to_key
import re
import sys
import traceback
sys.path.append(".")

from advent import Advent
#fmt: on

digits = re.compile("\d+")


def to_list(input):
    if input[0] != '[':
        raise Exception("Invalid list at " + input)
    input = input[1:]

    result = []
    while True:
        if input[0] == "]":
            return (input[1:], result)
        if input[0] == "[":
            try:
                (input, sub) = to_list(input)
                result.append(sub)
            except:
                print((input, result))
                raise
        else:
            num = digits.match(input)
            result.append(int(num[0]))
            input = input[num.end():]
        if input[0] == ",":
            input = input[1:]


def compare(first, second):
    if type(first) is int:
        if type(second) is int:
            if first < second:
                return 1
            if first > second:
                return -1
            return 0
        first = [first]
    elif type(second) is int:
        second = [second]

    for i, s in enumerate(second):
        if i == len(first):
            return 1
        f = first[i]
        cmp = compare(f, s)
        if cmp != 0:
            return cmp

    if len(second) < len(first):
        return -1
    return 0


class Day(Advent):
    day = "13"

    def parseState(self, f):
        self.last = ""
        self.sum = 0
        self.index = 0
        self.packets = []

    def parseCommand(self, line: str):
        if line > "" and self.last > "":
            return (self.last, line)
        self.last = line

    def process1(self, command):
        if command is None:
            return

        (_, list1) = to_list(command[0])
        (_, list2) = to_list(command[1])

        self.index += 1
        cmp = compare(list1, list2)
        if cmp > 0:
            self.sum += self.index

    def result1(self):
        return self.sum

    def process2(self, command):
        if command is not None:
            self.packets += [to_list(c)[1] for c in command]

    def result2(self):
        two = [[2]]
        six = [[6]]

        before_2 = []
        before_6 = []
        after_6 = []

        for p in self.packets:
            if compare(p, two) > 0:
                before_2.append(p)
            elif compare(p, six) > 0:
                before_6.append(p)
            else:
                after_6.append(p)
                
        if len(self.packets) != (len(before_2) + len(before_6) + len(after_6)):
            raise Exception("Packets missing")

        i_2 = len(before_2) + 1
        i_6 = i_2 + len(before_6) + 1

        return i_2 * i_6


day = Day()

try:
    day.test1(13)
    day.pass1()

    day.test2(140)
    day.pass2()
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
