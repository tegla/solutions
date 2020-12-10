from functools import lru_cache

with open('/tmp/input.txt') as f:
    js = [int(l) for l in f]

js.sort()
js.append(js[-1]+3)
js.insert(0,0)

combinations = [0]*len(js)
combinations[-1] = 1

def compute(i):
    c = 0
    n = i + 1
    while n < len(js) and js[n] <= js[i] + 3:
        c += combinations[n]
        n += 1
    combinations[i] = c

for i in range(len(js)-2, -1, -1):
    compute(i)

print(combinations)