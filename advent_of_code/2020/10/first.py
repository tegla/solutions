import functools

with open('/tmp/input.txt') as f:
    js = [int(l) for l in f]

js.sort()
js.append(js[-1]+3)
js.insert(0,0)

# This code is clever, but not really readable. Should have gone with procedural.
def dictadd(a,b):
    return dict([(k, a.get(k,0) + b.get(k,0)) for k in a.keys() | b.keys()])

diffs = functools.reduce(dictadd, [{b-a:1} for a,b in zip(js,js[1:])])

print(diffs)
print(diffs[1] * diffs[3])