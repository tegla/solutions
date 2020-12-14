from functools import reduce
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

def setconcat(s, suffices):
    return set([s + suffix for suffix in suffices])

def minus(a,b):
    if a == b:
        return set()
    ha = a[0]
    hb = b[0]
    ta = a[1:]
    tb = b[1:]
    tm = minus(ta,tb)
    if ha == hb:
        return setconcat(ha, tm)
    elif ha == 'X':
        if {ta} == tm:
            return {'X' + ta}
        elif hb == '1':
            return set(['0' + ta]).union(setconcat('1', tm))
        elif hb == '0':
            return set(['1' + ta]).union(setconcat('0', tm))
        else:
            assert False
    elif ha == '1' and hb == 'X':
        return setconcat('1', tm)
    elif ha == '0' and hb == 'X':
        return setconcat('0', tm)
    elif ha == '0' and hb == '1':
        return {a}
    elif ha == '1' and hb == '0':
        return {a}
    else:
        assert False, (a,b)

assert minus('X1101X','01X0XX') == {'11101X'}
assert minus('XXX', '000') == {'001', '01X', '1XX'}

def setminus(xs, b):
    return reduce(lambda res,x: res.union(minus(x,b)), xs, set())
    
def countset(ss):
    return sum([2**(s.count('X')) for s in ss])

memory = dict()

for mask, mems in ops:
    print("mask ", mask)
    for addr, v in mems:
        sm = submask(mask, addr)
        print(v,sm)
        for otherv in memory.keys():
            memory[otherv] = setminus(memory[otherv], sm)
        if v in memory:
            memory[v] = memory[v].union({sm})
        else:
            memory[v] = {sm}

print(sum([countset(m)*v for v,m in memory.items()]))
