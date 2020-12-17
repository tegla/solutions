from itertools import count
from collections import defaultdict

m = set()


with open('/tmp/input.txt') as f:
    for x,l in zip(count(0), f):
        for y,c in zip(count(0), l.strip()):
            if c == '#':
                m.add((x,y,0))


def neighbours(p):
    (x,y,z) = p
    for dx in [-1,0,1]:
        for dy in [-1,0,1]:
            for dz in [-1,0,1]:
                if dx != 0 or dy != 0 or dz != 0:
                    yield (x+dx,y+dy,z+dz)


def newactive(p):
    n = [p2 in m for p2 in neighbours(p)].count(True)
    a = p in m
    if a and (n in [2,3]):
        return True
    if (not a) and n == 3:
        return True
    return False

print(m)
for r in range(0,6):
    m2 = set()
    for p in m:
        for p2 in neighbours(p):
            if newactive(p2):
                m2.add(p2)
    print(r+1, len(m2))
    m = m2

