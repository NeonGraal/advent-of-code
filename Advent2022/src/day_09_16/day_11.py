#fmt: off
import sys
import traceback
sys.path.append(".")

from advent import Advent
from itertools import islice
#fmt: on


class Monkey:
    id = 0
    items = []
    operator = " "
    operand = 0
    divisor = 1
    ifTrue = 0
    ifFalse = 0
    inspected = 0

    def __str__(self):
        return (f"#{self.id}: {self.items}" +
                f" -> {self.operator} {self.operand}" +
                f" if % {self.divisor} then {self.ifTrue} else {self.ifFalse}")

    def increase(self, i):
        if self.operator == "+":
            return i + self.operand
        if self.operator == "*":
            return i * self.operand
        if self.operator == "^":
            return i ** self.operand


class Day(Advent):
    day = "11"

    def parseState(self, f):
        self.monkeys = []

        while True:
            m = Monkey()
            line = f.readline()
            m.id = line.split()[1].rstrip(":")
            line = f.readline()
            m.items = [int(i.rstrip(",")) for i in line.split()[2:]]
            line = f.readline()
            parts = line.split()
            if parts[4] == "*" and parts[-1] == "old":
                m.operator = "^"
                m.operand = 2
            else:
                m.operator = parts[4]
                m.operand = int(parts[-1])
            line = f.readline()
            m.divisor = int(line.split()[-1])
            line = f.readline()
            m.ifTrue = int(line.split()[-1])
            line = f.readline()
            m.ifFalse = int(line.split()[-1])

            self.monkeys += [m]
            line = f.readline()
            if len(line) < 1:
                break

    def __str__(self):
        return "\n".join([str(m) for m in self.monkeys])

    def round(self, reduce, lcm):
        for m in self.monkeys:
            self.turn(m, reduce, lcm)

    def turn(self, m, reduce, lcm):
        for i in m.items:
            m.inspected += 1
            i = m.increase(i) // reduce % lcm
            if i % m.divisor == 0:
                self.monkeys[m.ifTrue].items += [i]
            else:
                self.monkeys[m.ifFalse].items += [i]
        m.items = []

    def result1(self):
        lcm = 1
        for m in self.monkeys:
            lcm *= m.divisor
        for _ in range(20):
            self.round(3, lcm)
        ins = [m.inspected for m in self.monkeys]
        ins.sort()
        return ins[-1] * ins[-2]

    def result2(self):
        lcm = 1
        for m in self.monkeys:
            lcm *= m.divisor
        for i in range(10000):
            self.round(1, lcm)
            if i % 100 == 0:
                print(".", end='')
        print("")
        ins = [m.inspected for m in self.monkeys]
        ins.sort()
        return ins[-1] * ins[-2]


day = Day()

try:
    day.test1(10605)
    day.pass1()

    day.test2(2713310158)
    day.pass2()
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
