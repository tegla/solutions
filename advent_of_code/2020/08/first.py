import re

def parseline(l):
    l = l.rstrip()
    op, num = re.fullmatch('([a-z]{3}) (.+)',l).groups()
    return (op, int(num))

with open('/tmp/input.txt') as f:
    ops = [parseline(l) for l in f]

acc = 0
seen = set()
ip = 0
while True:
    assert ip >=0
    assert ip < len(ops)
    print("ip=%d, acc=%d"%(ip,acc))
    if ip in seen:
        print(acc)
        break
    seen.add(ip)
    op, num = ops[ip]
    print("   op=%s, num=%d"%(op,num))
    if op == 'acc':
        acc += num
        ip += 1
    elif op == 'jmp':
        ip += num
    elif op == 'nop':
        ip += 1
    else:
        assert False, ip
