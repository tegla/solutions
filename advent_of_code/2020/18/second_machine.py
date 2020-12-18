def v(s):
    vals = []
    ops = ['(']
    prios = '(+*)'

    for c in s+')':
        if c == ' ':
            pass
        elif c in ['*', '+', '(', ')']:
            while True:
                op0 = ops[-1]
                if op0 == '(':
                    if c == ')':
                        ops.pop()
                    else:
                        ops.append(c)
                    break
                if prios.find(op0) <= prios.find(c):
                    ops.pop()
                    if op0 == '*':
                        vals.append(vals.pop() * vals.pop())
                    elif op0 == '+':
                        vals.append(vals.pop() + vals.pop())
                    continue
                ops.append(c)
                break
        else:
            vals.append(int(c))
    assert len(vals) == 1, vals
    return vals[0]

with open('/tmp/input.txt') as f:
    print(sum([v(l.strip()) for l in f]))
