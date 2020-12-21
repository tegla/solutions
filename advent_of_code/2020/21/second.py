from itertools import chain

foods = []

with open('/tmp/input.txt') as f:
    for line in f:
        l, r = line.split(" (contains ")
        l = l.split(" ")
        r = r[:-2].split(", ")
        foods.append((set(l), set(r)))

all_a = set(chain(*[al for (_, al) in foods]))
all_f = set(chain(*[fl for (fl, _) in foods]))

f_a = dict()
for f in all_f:
    s = set(all_a)
    for (fl, al) in foods:
        for a in al:
            if f not in fl:
                s.discard(a)
    if len(s) > 0:
        print(f, s)
        f_a[f] = s


f_a = list(f_a.items())
f_a.sort(key=lambda x: len(x[1]))


def mapping(known, rest):
    if len(rest) == 0:
        yield known
        return
    f, al = rest[0]
    al = set(al)
    for a in known.values():
        al.discard(a)
    for a in al:
        k = dict(known)
        k[f] = a
        yield from mapping(k, rest[1:])


m = list(mapping({}, f_a))
assert len(m) == 1
m = m[0]
m = list(m.items())
m.sort(key=lambda x: x[1])

print(",".join([f for f, a in m]))
