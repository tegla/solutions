import re

def bin(n):
    return '{0:b}'.format(n)

def parse():
    with open('/tmp/input.txt') as f:
        mems = []
        for l in f:
            l = l.strip()
            mask = re.fullmatch("mask = (.*)", l)
            if mask:
                if len(mems) > 0:
                    yield (m, n, mems)
                    mems = []
                n = mask.group(1)
                m = int(n.replace(
                    '1', '0').replace('X', '1'), 2)
                n = int(n.replace('X', '0'), 2)
            else:
                mem = re.fullmatch("mem\[(\d+)\] = (.*)", l)
                mems.append((int(mem.group(1)), int(mem.group(2))))
        yield (m, n, mems)


ops = list(parse())
