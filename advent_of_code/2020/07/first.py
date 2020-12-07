from input import bs

rev = {}
for b,os in bs:
    for n,o in os:
        if o not in rev:
            rev[o] = set()
        rev[o].add(b)

rs = set()
q = set(['shiny gold'])
while len(q) > 0:
    b = q.pop()
    rs.add(b)
    for r in rev.get(b, []):
        if r not in rs and r not in q:
            q.add(r)

# 'shiny gold' itself is not a solution, so ignore that
print(len(rs)-1)