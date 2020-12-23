
c = list(map(int, list("398254716")))

for r in range(1,101):
    print(r,c)
    ts = c[1:4]
    c[1:4] = []
    d = (c[0])
    while True:
        d-=1
        if d == 0:
            d = 9
        if d not in ts:
            break
    i = c.index(d)
    print("  -> ", d, i, ts, c)
    c[i+1:i+1] = ts
    c = c[1:] + [c[0]]
    print("  ->", c)

while c[0] != 1:
    c = c[1:] + [c[0]]

print("".join(map(str,c[1:])))