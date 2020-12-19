from collections import defaultdict


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
            rs = [rs[1]]
        else:
            rs = rs.strip().split("|")
            rs = [list(map(int, r.strip().split(" "))) for r in rs]
        rules[n] = rs

    for l in f:
        messages.append(l.strip())

rules[8] = [[42], [42, 8]]
rules[11] = [[42, 31], [42, 11, 31]]


def goodmessage(m):
    # string position, rule_nr -> set of lengths that match
    matches = defaultdict(set)

    def MatchRuleListLen(i, rl):
        if isinstance(rl, str):
            if m[i:i+len(rl)] == rl:
                yield len(rl)
            return
        if len(rl) == 0:
            yield 0
            return
        for l in matches[(i, rl[0])]:
            for restlen in MatchRuleListLen(i+l, rl[1:]):
                yield l + restlen

    while True:
        updated = False
        for n, rs in rules.items():
            if isinstance(rs, str):
                continue
            for rl in rs:
                for i in range(0, len(m)):
                    for l in MatchRuleListLen(i, rl):
                        ls = matches[(i, n)]
                        if not l in ls:
                            updated = True
                            matches[(i, n)].add(l)
        if not updated:
            break
    return len(m) in matches[(0, 0)]


c = 0
for m in messages:
    g = goodmessage(m)
    if (g):
        c += 1
    print(m, g)

print(c)
