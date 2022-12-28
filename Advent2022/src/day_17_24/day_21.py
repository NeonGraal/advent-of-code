#fmt: off
import sys
import traceback
sys.path.append(".")

from advent import Advent
#fmt: on


class Monkey:
    def __init__(self, *parts: list[str]) -> None:
        self.id = parts[0]
        self.value = None
        self.left = None
        self.op = None
        self.right = None
        
        if len(parts) == 2:
            self.value = int(parts[1])
        else:
            self.left = parts[1]
            self.op = parts[2]
            self.right = parts[3]
            
    def __repr__(self) -> str:
        if self.value != None:
            return f"{self.id}: {self.value}"
        return f"{self.id}: {self.left} {self.op} {self.right}"
    
    def expr(self)-> str:
        if self.value != None:
            return f"{self.id}"
        return f"({self.left} {self.op} {self.right})"

    def calc(self, left: int, right: int):
        if self.value != None:
            return
        
        match self.op:
            case "+":
                self.value = left + right
            case "-":
                self.value = left - right
            case "*":
                self.value = left * right
            case "/":
                self.value = left // right
            case _:
                raise Exception(f"Unknown operation: {self}")


class Day(Advent):
    day = "21"

    def parseState(self, f):
        self.monkeys = {}

    def parseCommand(self, line: str) -> Monkey:
        return Monkey(*[s.strip(':') for s in line.split()])
    
    def process1(self, monkey: Monkey):
        self.monkeys[monkey.id] = monkey
        
    def __str__(self) -> str:
        return "\n".join([str(m) for m in self.monkeys.values()])
    
    def calculate(self, key: str) -> int:
        monkey = self.monkeys[key]
        if monkey.value == None:
            monkey.calc(self.calculate(monkey.left), self.calculate(monkey.right))
        return monkey.value

    def result1(self):
        return self.calculate("root")
    
    def derive(self, key: str) -> Monkey:
        monkey = self.monkeys[key]
        if monkey.value != None:
            return monkey
        
        left = self.derive(monkey.left)
        right = self.derive(monkey.right)
        
        if left.id == "humn":            
            return Monkey("humn", f"{left.expr()}", monkey.op, right.value)
        
        if right.id == "humn":
            if right.value != None:
                return Monkey("humn", left.value, monkey.op, f"{right.expr()}")
            match monkey.op:
                case "*":                    
                    return Monkey("humn", left.value, monkey.op, f"{right.expr()}")
                case _:
                    return Monkey("humn", left.value, monkey.op, f"{right.expr()}")
        
        monkey.calc(left.value, right.value)
        return monkey

    def result2(self):
        root = self.derive("root")
        
        print(root.expr())
        
        return 0

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
