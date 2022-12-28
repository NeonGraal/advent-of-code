#fmt: off
import sys
import traceback
sys.path.append(".")

from advent import Advent
#fmt: on


class Cube:
    def __init__(self, *parts):
        (self.x, self.y, self.z) = parts[:3]
        self.sides = 6
        
    def __repr__(self) -> str:
        return f"{self.x},{self.y},{self.z}"
    
    def adjacent(self, cubes, key):
        if key not in cubes:
            return
        self.sides -= 1
        cubes[key].sides -= 1
    
    def check(self, cubes):
        for d in [1, -1]:
            self.adjacent(cubes, f"{self.x+d},{self.y},{self.z}")
            self.adjacent(cubes, f"{self.x},{self.y+d},{self.z}")
            self.adjacent(cubes, f"{self.x},{self.y},{self.z+d}")

class Day(Advent):
    day = "18"
    
    def parseState(self, f):
        self.cubes = {}
    
    def parseCommand(self, line: str):
        cube = Cube(*[int(s) for s in line.split(",")])
        cube.check(self.cubes)
        self.cubes[str(cube)] = cube
        
    def result1(self):
        return sum([c.sides for c in self.cubes.values()])

day = Day()

try:
    day.test1(64)
    day.pass1()

    day.test2(58)
    day.pass2()
except:
    ex = sys.exc_info()
    print(day)
    traceback.print_exception(*ex)
