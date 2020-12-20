from collections import defaultdict
from collections import Counter
import itertools


MONSTER = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   ",
]

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
    x, y = p
    return (SIZE-x-1, y)


def rot(p):
    x, y = p
    return (SIZE-y-1, x)


def at(tile, trs, p):
    for tr in trs:
        p = tr(p)
    x, y = p
    return tile[x][y]


def read(tile, trs, pit):
    for p in pit:
        yield at(tile, trs, p)


TOP = [(0, y) for y in range(SIZE)]
BOTTOM = [(SIZE-1, y) for y in range(SIZE)]
LEFT = [(x, 0) for x in range(SIZE)]
RIGHT = [(x, SIZE-1) for x in range(SIZE)]

TRS = [
    [],
    [rot],
    [rot, rot],
    [rot, rot, rot],
    [flipx],
    [flipx, rot],
    [flipx, rot, rot],
    [flipx, rot, rot, rot],
]


def edge(tile, trs, pit):
    return "".join(read(tile, trs, pit))


edge_to_tile = defaultdict(set)
for n, t in tiles.items():
    for trs in TRS:
        edge_to_tile[edge(t, trs, TOP)].add(n)

nomatch = [e for e, ns in edge_to_tile.items() if len(ns) == 1]


def ptrs(trs):
    return ",".join([tr.__name__ for tr in trs])

# Now build the new map


def match(n, trs, side, otherside):
    e = edge(tiles[n], trs, side)
    for on in edge_to_tile[e]:
        if on != n:
            for otrs in TRS:
                if e == edge(tiles[on], otrs, otherside):
                    return (on, otrs)
    return (None, None)


# Let's determine a top left corner
for topleft, t in tiles.items():
    if [edge(t, [], pit) in nomatch for pit in [TOP, BOTTOM, LEFT, RIGHT]].count(True) != 2:
        continue
    print("Using topleft:", topleft)

    # Let's determine an orientation
    for topleft_trs in TRS:
        if edge(tiles[topleft], topleft_trs, TOP) in nomatch and edge(tiles[topleft], topleft_trs, LEFT) in nomatch:
            print("Using topleft TRS:", ptrs(trs))

            m = []
            left = topleft
            left_trs = topleft_trs
            while left is not None:
                maprows = []
                for x in tiles[left][1:-1]:
                    maprows.append([])
                middle, middle_trs = left, left_trs
                while middle is not None:
                    for x in range(1, SIZE-1):
                        for y in range(1, SIZE-1):
                            maprows[x-1].append(at(tiles[middle],
                                                   middle_trs, (x, y)))
                    middle, middle_trs = match(middle, middle_trs, RIGHT, LEFT)
                left, left_trs = match(left, left_trs, BOTTOM, TOP)
                for r in maprows:
                    m.append("".join(r))

            # Now find the monsters!
            foundmonster = 0
            for x in range(0, len(m)-len(MONSTER)):
                for y in range(0, len(m[0])-len(MONSTER[0])):
                    matches = True
                    for i in range(0, len(MONSTER)):
                        for j in range(0, len(MONSTER[i])):
                            if MONSTER[i][j] == '#':
                                if m[x+i][y+j] != '#':
                                    matches = False
                    if matches:
                        foundmonster += 1
                        print("matches ", x, y)
                        m2 = [list(r) for r in m]
                        for i in range(0, len(MONSTER)):
                            for j in range(0, len(MONSTER[i])):
                                if MONSTER[i][j] == '#':
                                    m2[x+i][y+j] = 'O'
                        m = ["".join(l) for l in m2]
            wavecount = sum([l.count("#") for l in m])
            print("found monsters", foundmonster, wavecount)
            print()
