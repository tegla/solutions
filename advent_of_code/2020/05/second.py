from functools import reduce


with open('/tmp/input.txt') as f:
    ls = [l.rstrip() for l in f]

def multiplier(c):
    return 1 if c in {'B','R'} else 0

def compute(s):
    return reduce(lambda a,b:a*2+b, map(multiplier, s))

def row(l):
    return compute(l[:7])

def col(l):
    return compute(l[7:])

def id(l):
    return row(l) * 8 + col(l)

assert id('BFFFBBFRRR') == 567
assert id('FFFBBBFRRR') == 119
assert id('BBFFBBFRLL') == 820

ids = list(map(id,ls))
ids.sort()

for l,r in zip(ids, ids[1:]):
    if r == l+2:
        print(l+1)
