import re

bs = []
with open('/tmp/input.txt') as f:
    for l in f:
        l = l.rstrip()
        m = re.fullmatch('(.*) bags contain (.*).', l)
        assert m, l
        b = m.group(1)
        o = []
        if m.group(2) != 'no other bags':
            for s in m.group(2).split(', '):
                m = re.fullmatch('(\d+) (.*) bag(s)?', s)
                assert m, l
                o.append((int(m.group(1)), m.group(2)))
        bs.append((b,o))


        