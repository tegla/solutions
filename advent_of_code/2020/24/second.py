from collections import defaultdict
from functools import reduce
from itertools import chain

DIRS = ["nw", "ne", "sw", "se", "w", "e"]
PTRS = [(-1, -1), (-1, 0), (1, -1), (1, 0), (0, -1), (0, 1)]
sm = dict(zip(DIRS, PTRS))


def parse(l):
    dl = []
    l = l.strip()
    while len(l) > 0:
        for d in DIRS:
            if l.startswith(d):
                dl.append(l[0:len(d)])
                l = l[len(d):]
                break
    return dl


with open('/tmp/input.txt') as f:
    dll = list(map(parse, f))


def step(p, d):
    x, y = p
    dx, dy = sm[d]
    x += dx
    y += dy
    if x % 2 == 0 and len(d) > 1:
        y += 1
    return (x, y)


def go(p, dl):
    return reduce(step, dl, p)


assert go((0, 0), parse('nwwswee')) == (0, 0)

m = defaultdict(bool)

for dl in dll:
    p = go((0, 0), dl)
    m[p] = not(m[p])


def surrounding(p):
    return [step(p, d) for d in DIRS]


print(list(m.values()).count(True))

for r in range(1, 100+1):
    m2 = defaultdict(bool)
    all_possible = set(chain(*map(surrounding, m.keys())))
    for p in all_possible:
        black = m[p]
        s_black = [m[s] for s in surrounding(p)].count(True)
        if black and (s_black == 0 or s_black > 2):
            pass
        elif not black and s_black == 2:
            m2[p] = True
        elif black:
            m2[p] = True
    m = m2

    print(r, list(m.values()).count(True))
