
rules = dict()
messages = []

with open('/tmp/input.txt') as f:
    for l in f:
        l = l.strip()
        if l == "":
            break
        n, rs = l.split(":")
        rs = rs.strip()
        n = int(n)
        if rs[0] == '"':
            rs = rs[1]
        else:
            rs = rs.strip().split("|")
            rs = [list(map(int, r.strip().split(" "))) for r in rs]
        rules[n] = rs

    for l in f:
        messages.append(l.strip())


def matches(m, n):
    if len(m) == 0:
        return (False, 0)
    r = rules[n]
    if isinstance(r, str):
        return (m.startswith(r), m[len(r):])
    for rs in r:
        ma = True
        rest = m
        for ra in rs:
            ma, rest = matches(rest, ra)
            if not ma:
                break
        if ma:
            return (True, rest)
    return (False, "")


def goodmessage(m):
    ma, rest = matches(m, 0)
    return ma and rest == ""


c = 0
for m in messages:
    g = goodmessage(m)
    if (g):
        c += 1
    print(m, g)

print(c)
