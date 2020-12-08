import re

def parseline(l):
    l = l.rstrip()
    op, num = re.fullmatch('([a-z]{3}) (.+)',l).groups()
    return (op, int(num))

with open('/tmp/input.txt') as f:
    ops = [parseline(l) for l in f]

def run(flipped):
    acc = 0
    seen = set()
    ip = 0
    while True:
        assert ip >=0
        assert ip <= len(ops)
        if ip == len(ops):
            return acc
        # print("ip=%d, acc=%d"%(ip,acc))
        if ip in seen:
            return None
        seen.add(ip)
        op, num = ops[ip]
        # print("   op=%s, num=%d"%(op,num))
        if ip == flipped:
            if op == 'jmp':
                op = 'nop'
            elif op == 'nop':
                op = 'jmp'
        if op == 'acc':
            acc += num
            ip += 1
        elif op == 'jmp':
            ip += num
        elif op == 'nop':
            ip += 1
        else:
            assert False, ip

for i in range(0, len(ops)):
    acc = run(i)
    if acc is not None:
        print(acc)
