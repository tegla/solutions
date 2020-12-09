from itertools import combinations

with open('/tmp/input.txt') as f:
    ns = [int(l) for l in f]

PREFIX=25

def valid(i):
    return ns[i] in [sum(c) for c in combinations(ns[i-PREFIX:i],2)]

for i in range(PREFIX, len(ns)):
    if not valid(i):
        print(ns[i])
        break
else:
    print("not found")