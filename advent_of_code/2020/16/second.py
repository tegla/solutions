from input import *

import operator
from functools import reduce
from collections import defaultdict


def validforfield(field, num):
    (_, l1, r1, l2, r2) = field
    res = (l1 <= num and num <= r1) or (l2 <= num and num <= r2)
    #print(field, num, res)
    return res


def validforafield(num):
    return reduce(operator.or_, [validforfield(f, num) for f in fields])


def validticket(t):
    return reduce(operator.and_, [validforafield(n) for n in t])


theirs = [t for t in theirs if validticket(t)]


def validforcol(field, col):
    return reduce(operator.and_, [validforfield(field, t[col]) for t in theirs])


validcols = defaultdict(set)
for f in range(0, len(fields)):
    for c in range(0, len(mine)):
        if validforcol(fields[f], c):
            validcols[f].add(c)

orderedvalidcols = list(validcols.items())
orderedvalidcols.sort(key=lambda x: len(x[1]))

seen = set()
fieldmap = dict()

for f, vc in orderedvalidcols:
    left = vc - seen
    # This is a greedy search with no backtracking.
    # So if there are more than one option, bail out.
    assert len(left) == 1
    c = left.pop()
    fieldmap[f] = c
    seen.add(c)

res = 1
for i in range(0, len(fields)):
    (name, _, _, _, _) = fields[i]
    if name.startswith("departure"):
        res *= mine[fieldmap[i]]

print(res)
