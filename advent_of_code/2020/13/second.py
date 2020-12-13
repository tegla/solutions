from functools import reduce
with open('/tmp/input.txt') as f:
    ts = int(next(f))
    buses = [None if n == 'x' else int(n) for n in next(f).strip().split(',')]

print(buses)
os = []
for i in range(0, len(buses)):
    if buses[i] is not None:
        os.append((buses[i], i))

print(os)


def inverse(a, p):
    # Fermat's algorithm for modular inverse (all bus ids are prime)
    res = 1
    for i in range (0, p-2):
        res*=a
        res%=p
    return res

def common(b1, b2):
    print(b1, b2)
    m1, o1 = b1
    m2, o2 = b2
    i = (inverse(m1, m2)*(-o1-o2))%m2
    return (m1*m2, m1*i+o1)


m, o = reduce(common, os)
print("result: ", o)
