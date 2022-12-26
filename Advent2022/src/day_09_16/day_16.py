#fmt: off
import sys
import traceback
sys.path.append(".")

from advent import Advent
#fmt: on


class Valve:
    def __init__(self, line):
        parts = line.split()
        self.id = parts[1]
        self.flow = int(parts[4].split("=")[1][:-1])
        self.tunnels = [p.rstrip(',') for p in parts[9:]]
        self.valves = {}

    def __str__(self) -> str:
        valves = [f"{k}:{self.valves[k]}" for k in sorted(self.valves)]
        return f"{self.id} {self.flow} - {self.tunnels} - {valves}"


class State1:
    def __init__(self, time: int, current: Valve, released: int, open: dict):
        self.time = time
        self.current = current
        self.released = released
        self.open = open

    def tick(self, valves, max):
        if self.time >= 30:
            return

        curr = self.current
        time = self.time + 1
        released = self.released + (30 - time) * curr.flow
        open = dict(self.open)
        open[curr.id] = time

        if released > max:
            yield State1(30, curr, released, open)

        paths = [v for v in curr.valves if v not in open]
        for p in paths:
            time = self.time + curr.valves[p] + 1
            if time <= 30:
                yield State1(time, valves[p], released, open)

    def __str__(self) -> str:
        return f"#{self.time} @{self.current.id} ~{self.released} - {self.open}"

    def sort(self):
        return self.time - self.released


class State2:
    def __init__(self, time: int, opening: Valve, other_time: int, other: Valve, released: int, open: dict):
        self.time = time
        self.opening = opening
        self.other_time = other_time
        self.other = other
        self.released = released
        self.open = open

    def sort(self):
        return self.time - self.other_time - self.released

    def tick(self, valves, max):
        if self.time >= 26:
            return

        curr = self.opening
        time = self.time + 1
        released = self.released + (26 - time) * curr.flow

        if self.other_time < 26 or released > max:
            yield State2(self.other_time, self.other, 26, curr, released, self.open)

        paths = [v for v in curr.valves if v not in self.open]
        for p in paths:
            time = self.time + curr.valves[p] + 1
            if time > 26:
                continue
            open = dict(self.open)
            open[p] = time
            if time < self.other_time:
                yield State2(time, valves[p], self.other_time, self.other, released, open)
            else:
                yield State2(self.other_time, self.other, time, valves[p], released, open)

    def __str__(self) -> str:
        return f"#{self.time} @{self.opening.id} (#{self.other_time} @{self.other.id}) ~{self.released} - {self.open}"


class Day(Advent):
    day = "16"

    def parseState(self, f):
        self.valves = {}
        self.costs = {}

    def parseCommand(self, line: str):
        return Valve(line)

    def add_cost(self, key, p1, p2):
        if p1 in self.costs and p2 in self.costs:
            cost = self.costs[p1] + self.costs[p2]
            if key in self.costs:
                if cost < self.costs[key]:
                    self.costs[key] = cost
            else:
                self.costs[key] = cost

    def process1(self, v):
        self.costs[(v.id, v.id)] = 0
        for t in v.tunnels:
            self.costs[(v.id, t)] = 1
        for o in self.valves.values():
            for t in v.tunnels:
                self.add_cost((o.id, t), (o.id, v.id), (v.id, t))
                self.add_cost((v.id, o.id), (v.id, t), (t, o.id))
            for t in o.tunnels:
                self.add_cost((v.id, t), (v.id, o.id), (o.id, t))
                self.add_cost((o.id, v.id), (o.id, t), (t, v.id))

        self.valves[v.id] = v

    # def __str__(self) -> str:
        # return "\n".join((f"{v}" for v in self.valves.values()))

    def finish_valves(self):
        needed = []
        for x in self.valves:
            for y in self.valves:
                if (x, y) not in self.costs:
                    needed.append((x, y))

        while len(needed) > 0:
            k = needed.pop(0)
            curr = self.valves[k[0]]
            for t in curr.tunnels:
                self.add_cost(k, (k[0], t), (t, k[1]))
            if k not in self.costs:
                needed.append(k)

        for v in self.valves.values():
            v.valves = {}
            for o in self.valves.values():
                if o.flow == 0 or o.id == v.id:
                    continue
                v.valves[o.id] = self.costs[(v.id, o.id)]

    def result1(self):
        self.finish_valves()

        states = []
        start = self.valves["AA"]
        for v in start.valves:
            time = start.valves[v]
            states.append(State1(time, self.valves[v], 0, {v: time}))
        states.sort(key=State1.sort)

        max = states[0]
        i = 0

        while len(states) > 0:
            curr = states.pop(0)
            i += 1
            if not i % 100000:
                print(f"@{i} Best {max.released} Remaining {len(states)}")
            if curr.released > max.released:
                max = curr
                states = [s for s in states if s.time <
                          30 or s.released > max.released]
            states += [s for s in curr.tick(self.valves, max.released)]
            states.sort(key=State1.sort)

        print(max)
        return max.released

    def test_step(self, state: State2) -> list[State2]:
        print(state)
        print("--")
        states = [s for s in state.tick(self.valves)]
        states.sort(key=State2.sort)
        print("\n".join((str(s) for s in states)))
        print("--")
        return states

    def result2(self):
        self.finish_valves()

        states = []
        start = self.valves["AA"]
        for v1 in start.valves:
            time = start.valves[v1]
            for v2 in start.valves:
                other_time = start.valves[v2]
                if v1 == v2 or other_time < time:
                    continue
                states.append(State2(time, self.valves[v1], other_time, self.valves[v2], 0, {
                              v1: time, v2: other_time}))
        states.sort(key=State2.sort)

        # first = State2(start.valves["DD"], self.valves["DD"], start.valves["JJ"], self.valves["JJ"], 0, {
        #                "DD": start.valves["DD"], "JJ": start.valves["JJ"]})

        # next = self.test_step(first).pop(1)
        # self.test_step(next)
        # return

        max = states[0]
        i = 0

        while len(states) > 0:
            curr = states.pop(0)
            i += 1
            if not i % 1000000:
                print(f"@{i} Best {max.released} Remaining {len(states)} Current {curr}")
            if curr.released > max.released:
                max = curr
                states = [s for s in states if s.time <
                          26 or s.other_time < 26 or s.released > max.released]
            states += [s for s in curr.tick(self.valves, max.released)]
            states.sort(key=State2.sort)

        print(max)
        return max.released


day = Day()

try:
    day.test1(1651)
    day.pass1()

    day.test2(1707)
    day.pass2()
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
