with open('/tmp/input.txt') as f:
    ts = int(next(f))
    buses = [None if n == 'x' else int(n) for n in next(f).strip().split(',')]

print(ts)
print(buses)

nearest = min(filter(lambda x: x is not None, buses), key=lambda n: n - ts % n)
print(nearest)
print(nearest * (nearest - ts % nearest))
