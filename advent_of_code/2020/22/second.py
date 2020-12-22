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

def v(p):
    return sum([n*c for n, c in zip(count(len(p), -1), p)])

def combat(depth, p1,p2):
    p1 = deque(p1)
    p2 = deque(p2)
    #print("Combat:", depth, p1,p2)
    oldrounds = set()
    round = 0
    while len(p1) > 0 and len(p2) > 0:
        round += 1
        #print("Round: ", depth, round, p1,p2)
        state = (tuple(p1), tuple(p2))
        d1 = p1.popleft()
        d2 = p2.popleft()
        if state in oldrounds:
            return True
        elif len(p1) >= d1 and len(p2) >= d2:
            p1win = combat(depth+1, p1, p2)
        else:
            p1win = d1 > d2

        oldrounds.add(state)
        if p1win:
            p1.append(d1)
            p1.append(d2)
        else:
            p2.append(d2)
            p2.append(d1)
    #print("End Combat:", depth, p1,p2)
    if depth == 1:
        print(v(p1), v(p2))
    return len(p1) > 0

print(combat(1, p1,p2))