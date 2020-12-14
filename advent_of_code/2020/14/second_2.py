from collections import defaultdict
import re

def parse():
    with open('/tmp/input.txt') as f:
        mems = []
        for l in f:
            l = l.strip()
            mask = re.fullmatch("mask = (.*)", l)
            if mask:
                if len(mems) > 0:
                    yield (m, mems)
                    mems = []
                m = mask.group(1)
            else:
                mem = re.fullmatch("mem\[(\d+)\] = (.*)", l)
                mems.append((int(mem.group(1)), int(mem.group(2))))
        yield (m, mems)


ops = list(parse())

def submask(mask, num):
    res = []
    for (m,n) in zip(mask, '{0:036b}'.format(num)):
        if m == '0':
            res.append(n)
        elif m == '1':
            res.append('1')
        else:
            res.append('X')
    return ''.join(res)

assert submask('000000000000000000000000000000X1001X', 42) == '000000000000000000000000000000X1101X'
assert submask('00000000000000000000000000000000X0XX', 26) == '00000000000000000000000000000001X0XX'

def addresses(mask):
    if mask == '':
        yield 0
        return
    h = mask[0]
    if h == '0' or h == 'X':
        yield from addresses(mask[1:])
    if h == '1' or h == 'X':
        for a in addresses(mask[1:]):
            yield 2**(len(mask)-1) + a

memory = defaultdict(lambda:0)

for mask, mems in ops:
    print("mask ", mask)
    for addr, v in mems:
        sm = submask(mask, addr)
        print(sm,v)
        for a in addresses(sm):
            memory[a] = v

print(sum(memory.values()))
