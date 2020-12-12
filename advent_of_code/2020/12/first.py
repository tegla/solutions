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
heading = 1

pos = (0,0)

def add(p,d):
    return (p[0]+d[0], p[1]+d[1])

def mul(d, n):
    return (d[0]*n, d[1]*n)


for d,n in route:
    print(d,n)
    if d in dirs.keys():
        pos = add(pos, mul(dirs[d],n))
    elif d == 'L':
        heading -= (n//90)
        heading%=4
    elif d == 'R':
        heading += (n//90)
        heading%=4
    elif d == 'F':
        pos = add(pos, mul(dirs[clockwise[heading]], n))
    print(pos)

print(abs(pos[0])+abs(pos[1]))