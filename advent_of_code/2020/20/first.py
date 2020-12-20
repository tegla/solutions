from collections import defaultdict
from collections import Counter
tiles = defaultdict(list)

with open('/tmp/input.txt') as f:
    for l in f:
        l = l.strip()
        if l == "":
            continue
        if l.startswith("Tile"):
            tile = int(l[5:9])
            continue
        tiles[tile].append(l)

SIZE = 10

def flipx(p):
    x,y = p
    return (SIZE-x-1,y)

def rot(p):
    x, y = p
    return (SIZE-y-1, x)

def read(tile, trs, pit):
    for p in pit:
        for tr in trs:
            p = tr(p)
        x,y = p
        yield tile[x][y]

TOP = [(0,y) for y in range(SIZE)]
BOTTOM = [(SIZE-1, y) for y in range(SIZE)]
LEFT = [(x,0) for x in range(SIZE)]
RIGHT = [(x, SIZE-1) for x in range(SIZE)]

TRS = [
    [],
    [rot],
    [rot,rot],
    [rot,rot,rot],
    [flipx],
    [flipx, rot],
    [flipx, rot, rot],
    [flipx, rot, rot, rot],
]

possible_edges = []
for t in tiles.values():
    for trs in TRS:
        possible_edges.append("".join(read(t,trs,TOP)))

nomatch = [e for e,n in Counter(possible_edges).items() if n == 1]

r = 1
for n,t in tiles.items():
    if ["".join(read(t,[],pit)) in nomatch for pit in [TOP,BOTTOM,LEFT,RIGHT]].count(True) == 2:
        r*=n

print(r)
