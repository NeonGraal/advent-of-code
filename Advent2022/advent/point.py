class Point:
    def parse(str):
        parts = str.split(",")
        if len(parts) != 2:
            raise ValueError("Point can only be created from two numbers: " + str)
        return Point(int(parts[0]), int(parts[1]))
        
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __hash__(self):
        return hash(self.x + 999) * 3333 + hash(self.y + 999)

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def __repr__(self):
        return f"{self.x},{self.y}"

    def __add__(self, other):
        return Point(self.x + other.x, self.y + other.y)

    def __iadd__(self, other):
        self.x += other.x
        self.y += other.y
        return self
    
    def __sub__(self, other):
        return Point(self.x - other.x, self.y - other.y)
    
    def __len__(self):
        return abs(self.x) + abs(self.y)

    def __irshift__(self, other):
        (dx, dy) = (other.x - self.x, other.y - self.y)
        (ax, ay) = (abs(dx), abs(dy))
        if ax > 1 or ay > 1:
            if ax > 0:
                self.x += dx // ax
            if ay > 0:
                self.y += dy // ay
        return self
