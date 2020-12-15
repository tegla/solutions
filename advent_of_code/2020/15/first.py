import collections
import itertools

lastspoken = collections.defaultdict(lambda:None)

initial = [16,1,0,18,12,14,19]
N = 2020

spokenpos = None

for t in range(1, N+1):
    if t < len(initial)+1:
        spoken = initial[t-1]
    else:
        spoken = 0 if spokenpos is None else t-spokenpos-1
    print(t,spoken)
    spokenpos = lastspoken[spoken]
    lastspoken[spoken] = t


