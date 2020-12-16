import re

fields = []
theirs = []

with open('/tmp/input.txt') as f:
    for l in f:
        l = l.strip()
        if l == '':
            break
        (name, l1,r1,l2,r2) = re.fullmatch("(.*): (\d+)-(\d+) or (\d+)-(\d+)", l).groups()
        fields.append((name, int(l1),int(r1),int(l2), int(r2)))
    next(f)
    l = next(f).strip()
    mine = list(map(int, l.split(',')))
    next(f)
    next(f)
    for l in f:
        l = l.strip()
        theirs.append(list(map(int, l.split(','))))

