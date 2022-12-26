#fmt: off
import sys
import traceback
sys.path.append(".")

from advent import Advent
#fmt: on

def zero(input):
    return [i for i, v in enumerate(input) if v[1] == 0][0]

def normal(input):
    z = zero(input)
    return input[z:] + input[:z]

class Day(Advent):
    day = "20"

    def parseState(self, f):
        self.nums = []

    def parseCommand(self, line: str):
        self.nums += [int(line)]

    

    def shift1(self, num, input):
        p = input.index(num)
        v = num[1]
        (d, c) = (1, v)
        if v < 0:
            (d, c) = (-1, -v)

        l = len(input)
        result = list(input)

        for i in range(c):
            n = (p + d) % l
            (result[n], result[p]) = (result[p], result[n])
            p = n

        return result

    def shift(self, num, input):
        f = input.index(num)
        n = num[1]
        l = len(input)
        if n < 0:
            d = n % (1 - l)
        else:
            d = n % (l - 1)
        t = (f + d) % l
        if n > 0:
            t += 1

        if f < t:
            beg = input[:f]
            mid = input[f+1:t]
            end = input[t:]

            return beg + mid + [num] + end
        if t < f:
            beg = input[:t]
            mid = input[t:f]
            end = input[f+1:]

            return beg + [num] + mid + end

        return input

    def result1(self):
        mixed = list(enumerate(self.nums))
        l = len(mixed)

        for n in enumerate(self.nums):
            mixed = self.shift(n, mixed)

        p = zero(mixed)
        p1 = (p + 1000) % l
        p2 = (p + 2000) % l
        p3 = (p + 3000) % l

        return mixed[p1][1] + mixed[p2][1] + mixed[p3][1]

    def result2(self):
        nums = [(i, 811589153 * v) for i, v in enumerate(self.nums)]
        mixed = list(nums)
        l = len(mixed)

        for n in nums * 10:
            mixed = self.shift(n, mixed)

        p = zero(mixed)
        p1 = (p + 1000) % l
        p2 = (p + 2000) % l
        p3 = (p + 3000) % l

        return mixed[p1][1] + mixed[p2][1] + mixed[p3][1]

# 8839 is too high
# Not 2064, -20632, 1584, 19475


day = Day()


def check(n, test):
    result = day.shift(n, test)
    z = zero(result)
    normal = (result + result)[z:z+len(result)]
    result1 = day.shift1(n, test)
    z = zero(result1)
    normal1 = (result1 + result1)[z:z+len(result1)]
    if normal != normal1:
        raise Exception(f"{normal} not {normal1} for {n} in {test}")


test = list(enumerate([0, 1, -4, 3, -2, 5]))
check((1, 1), test)
check((3, 3), test)
check((4, -2), test)
check((2, -4), test)
check((5, 5), test)

test = list(enumerate([0, 11, -44, 33, -22, 55]))
check((1, 11), test)
check((3, 33), test)
check((4, -22), test)
check((2, -44), test)
check((5, 55), test)

try:
    day.test1(3)
    day.pass1()

    day.test2(1623178306)
    day.pass2()
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
