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

    def tick(self, valves):
        if self.time >= 30:
            return

        curr = self.current
        time = self.time + 1
        released = self.released + (30 - time) * curr.flow
        open = dict(self.open)
        open[curr.id] = time

        yield State1(30, curr, released, open)

        paths = [v for v in curr.valves if v not in open]
        for p in paths:
            time = self.time + curr.valves[p] + 1
            if time <= 30:
                yield State1(time, valves[p], released, open)

    def __str__(self) -> str:
        return f"#{self.time} @{self.current.id} ~{self.released} - {self.open}"

class State2:
    def __init__(self, time: int, opening: Valve, other: Valve, released: int, open: dict, flow_open: int):
        self.time = time
        self.released = released
        self.open = open
        self.flow_open = flow_open

    def tick(self, valves, max_open):
        if self.time >= 30 or self.flow_open == max_open:
            return

        curr = self.current
        time = self.time + 1
        flow_open = self.flow_open + curr.flow
        released = self.released + (30 - time) * curr.flow
        open = dict(self.open)
        open[curr.id] = time

        yield State1(30, curr, released, open, flow_open)

        paths = [v for v in curr.valves if v not in open]
        for p in paths:
            time = self.time + curr.valves[p] + 1
            if time <= 30:
                yield State1(time, valves[p], released, open, flow_open)

    def __str__(self) -> str:
        return f"#{self.time} @{self.current.id} ~{self.released} - {self.open} / {self.flow_open}"


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

    def __str__(self) -> str:
        return "\n".join((f"{v}" for v in self.valves.values()))

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
        states.sort(key=state_sort)

        max = states[0]
        i = 0

        while len(states) > 0:
            curr = states.pop(0)
            i += 1
            if not i % 100000:
                print(f"@{i} Best {max.released} Remaining {len(states)}")
            if curr.released > max.released:
                max = curr
            states += [s for s in curr.tick(self.valves)]
            states.sort(key=state_sort)

        print(max)
        return max.released
    
    def result2(self):
        self.finish_valves()
        
        


def state_sort(state: State1):
    return state.time - state.released


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
