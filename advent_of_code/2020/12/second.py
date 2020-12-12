import re

with open('/tmp/input.txt') as f:
    route = []
    for l in f:
        l = l.strip()
        (d, n) = re.fullmatch('(.)(.*)', l).groups()
        route.append((d,int(n)))

dirs = {
    'N': (-1,0),
    'S': (1,0),
    'E': (0,1),
    'W': (0,-1),
}
clockwise = ['N','E','S','W']
waypoint = (-1, 10)

pos = (0,0)

def add(p,d):
    return (p[0]+d[0], p[1]+d[1])

def mul(d, n):
    return (d[0]*n, d[1]*n)

def rot90(d):
    return (d[1], -d[0])

def rot90n(d,n):
    for i in range(0, n%4):
        d = rot90(d)
    return d


for d,n in route:
    print(d,n)
    if d in dirs.keys():
        waypoint = add(waypoint, mul(dirs[d], n))
    elif d == 'L':
        waypoint = rot90n(waypoint, -(n//90))
    elif d == 'R':
        waypoint = rot90n(waypoint, n//90)
    elif d == 'F':
        pos = add(pos, mul(waypoint, n))
    print(waypoint)
    print(pos)

print(abs(pos[0])+abs(pos[1]))