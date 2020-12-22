from collections import deque
from itertools import count

p1 = deque()
p2 = deque()

with open('/tmp/input.txt') as f:
    next(f)
    for l in f:
        l = l.strip()
        if l == "":
            break
        p1.append(int(l))
    next(f)
    for l in f:
        l = l.strip()
        p2.append(int(l))

print(p1)
print(p2)


while len(p2) > 0:
    if p1[0] < p2[0]:
        p1, p2 = p2, p1
    p1.append(p1.popleft())
    p1.append(p2.popleft())


p = list(p1)
p.reverse()
print(sum([n*c for n, c in zip(p, count(1))]))
