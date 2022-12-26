class Advent:
    day = "??"
    
    def parseState(self, f):
        pass

    def parseCommand(self, line: str):
        return line
    
    def process1(self, command):
        pass
    
    def result1(self):
        return None
    
    def process2(self, command):
        self.process1(command)
    
    def result2(self):
        return self.result1()

    def pass1(self):
        with open(f"input/day_{self.day}.txt", "r", encoding="utf-8") as f:
            self.parseState(f)

            for line in f:
                command = self.parseCommand(line.rstrip())
                self.process1(command)

        result = self.result1()
        print(f"Pass 1: {result}")

    def pass2(self):
        with open(f"input/day_{self.day}.txt", "r", encoding="utf-8") as f:
            self.parseState(f)

            for line in f:
                command = self.parseCommand(line.rstrip())
                self.process2(command)

        result = self.result2()
        print(f"Pass 2: {result}")

    def test1(self, expected):
        with open(f"sample/day_{self.day}.txt", "r", encoding="utf-8") as f:
            self.parseState(f)

            for line in f:
                command = self.parseCommand(line.rstrip())
                self.process1(command)

        result = self.result1()
        assert result == expected, f"Test 1: {result} != {expected}"
        print(f"Test 1: {expected}")

    def test2(self, expected, suffix = ""):
        with open(f"sample/day_{self.day}{suffix}.txt", "r", encoding="utf-8") as f:
            self.parseState(f)

            for line in f:
                command = self.parseCommand(line.rstrip())
                self.process2(command)

        result = self.result2()
        assert result == expected, f"Test 2{suffix}: {result} != {expected}"
        print(f"Test 2{suffix}: {expected}")
