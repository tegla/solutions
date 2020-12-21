from itertools import chain

foods = []

with open('/tmp/input.txt') as f:
    for line in f:
        l,r = line.split(" (contains ")
        l = l.split(" ")
        r = r[:-2].split(", ")
        foods.append((set(l),set(r)))

all_a = set(chain(*[al for (_,al) in foods]))
all_f = set(chain(*[fl for (fl,_) in foods]))

not_a = set()
for f in all_f:
    s = set(all_a)
    for (fl, al) in foods:
        for a in al:
            if f not in fl:
                s.discard(a)
    if len(s)==0:
        not_a.add(f)

c = 0
for (fl, _) in foods:
    for f in fl:
        if f in not_a:
            c+=1
print(c)
    