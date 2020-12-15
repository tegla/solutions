from collections import defaultdict, deque

# Map from number to the list of turns it was spoken (limited to last 2)
history = defaultdict(lambda:deque([],2))

initial = [16,1,0,18,12,14,19]
N=30000000

for t in range(1, N+1):
    if t < len(initial)+1:
        spoken = initial[t-1]
    else:
        h = history[spoken]
        if len(h) > 1:
            spoken = h[-1] - h[-2]
        else:
            spoken = 0
    history[spoken].append(t)

print(spoken)
