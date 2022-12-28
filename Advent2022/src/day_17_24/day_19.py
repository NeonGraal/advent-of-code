#fmt: off
import sys
import traceback
sys.path.append(".")

from advent import Advent
#fmt: on


class State:
    def __init__(self, time: int) -> None:
        self.robots = [1, 0, 0, 0]
        self.resources = [0, 0, 0, 0]
        self.time = time
        self.score = 0

    def calc(self, maximums):
        rem = 24 - self.time
        
        possible = [o + rem * r for o, r in zip(self.resources, self.robots)]
        possible[0] += rem * (rem + 1) / 2 / maximums[0]
        
        self.possible = self.resources[3] + rem * \
            self.robots[3] + rem * (rem + 1) / 2

        self.score = 100 * (
            100 * (100 * (100 * self.time + self.robots[3])
                   + self.robots[2]) + self.robots[1]) + self.robots[0]
        return self

    def build(self, robot, costs):
        result = State(self.time + 1)
        result.resources = [o + r - c for o, r,
                            c in zip(self.resources, self.robots, costs[robot])]
        result.robots = list(self.robots)
        if robot < 4:
            result.robots[robot] += 1

        return result

    def valid(self, maximums, max):
        return self.possible > max and all([r <= m * 2 for r, m in zip(self.resources, maximums)])
        # return all([o - r <= m for o, r, m in zip(self.resources, self.robots, maximums)])

    def __repr__(self) -> str:
        return f"@{self.time} : {self.resources} - {self.robots} ({self.score})"

    def key(self) -> str:
        return ",".join([self.time] + self.resources + self.robots)

    def by(self):
        return self.score


class Blueprint:
    def __init__(self, line: str) -> None:
        parts = [s.strip(':') for s in line.split()]
        nums = [int(v) for v in parts if v.isnumeric()]
        self.id = nums[0]
        self.costs = [
            (nums[1], 0, 0, 0),
            (nums[2], 0, 0, 0),
            (nums[3], nums[4], 0, 0),
            (nums[5], 0, nums[6], 0),
            (0, 0, 0, 0)
        ]
        self.maximums = (max(nums[1:4] + [nums[5]]), nums[4], nums[6], 99)
        self.rates = [
            (1+1/nums[1], 1, 1),
            (1+1/nums[2], 1, 1),
            (1+1/nums[3], 1+1/nums[4], 1),
            (1+1/nums[5], 1, 1+1/nums[6]),
        ]

    def __repr__(self) -> str:
        c = self.costs
        return f"#{self.id} Ore {c[0][0]} Clay {c[1][0]} Obsidian {c[2][0]},{c[2][1]} Geode {c[3][0]},{c[3][2]}  [{self.maximums}]"

    def afford(self, robot, resources) -> bool:
        if robot > 3:
            return True
        c = self.costs[robot]
        if robot < 2:
            return resources[0] >= c[0]
        return resources[0] >= c[0] and resources[1] >= c[1] and resources[2] >= c[2]

    def tick(self, state, max):
        for r in [4, 0, 1, 2, 3]:
            if self.afford(r, state.resources):
                next = state.build(r, self.costs).calc(self.maximums)
                if next.valid(self.maximums, max):
                    yield next

    def quality(self):
        states = [State(0).calc(self.maximums)]
        max = 0
        i = 0

        while len(states) > 0:
            curr = states.pop()
            if curr.possible <= max:
                continue
            if curr.time >= 24:
                if curr.resources[3] > max:
                    print((max, self.id, i, curr, len(states)))
                    max = curr.resources[3]
                continue

            states += list(self.tick(curr, max))

            i += 1
            if i % 100000 == 0:
                print((self.id, i, curr, len(states)))
                print("\n".join([f"- {s}" for s in states if s.time >= 24]))

            if i > 500000:
                print()
                return 0

        return self.id * max


class Day(Advent):
    day = "19"

    def parseState(self, f):
        self.blueprints = []

    def parseCommand(self, line: str):
        return Blueprint(line)

    def __repr__(self):
        all = [str(b) for b in self.blueprints]
        return "\n".join(all)

    def process1(self, blueprint):
        self.blueprints.append(blueprint)

    def result1(self):
        return sum([b.quality() for b in self.blueprints])


day = Day()

try:
    day.test1(33)
    day.pass1()

    day.test2(None)
    day.pass2()
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
