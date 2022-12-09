stacks = [""] * 9

def parseStacks(f):
    while (line := f.readline())[1] != "1":
        crates = line[1::4]
        for i, c in enumerate(crates):
            if c > " ":
                stacks[i] += c

def printStacks():            
    for i, s in enumerate(stacks):
        if s > "":  print(f"{i}: {s}")

def parseMove(line):
    parts = line.split()
    if len(parts) < 1 or parts[0] != "move":
        return (0, 0, 0)
    (cnt, frm, to) = [int(p) for p in parts[1::2]]
    frm -= 1
    to -= 1
    
    return (cnt, frm, to)

def processMove1(cnt, frm, to):
    for i in range(cnt):
        if stacks[frm] > "":
            stacks[to] = stacks[frm][0] + stacks[to]
            stacks[frm] = stacks[frm][1:]

def processMove2(cnt, frm, to):
    stacks[to] = stacks[frm][:cnt] + stacks[to]
    stacks[frm] = stacks[frm][cnt:]


with open("Advent2022/input/day_05.txt", "r", encoding="utf-8") as f:
    parseStacks(f)
    # printStacks()
                    
    for line in f:
        (cnt, frm, to) = parseMove(line)
        processMove2(cnt, frm, to)

    # printStacks()
    
for s in stacks:
    print(s[:1], end='')
print()