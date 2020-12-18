from collections import deque

def n(l):
    assert len(l) > 0, l
    c = l.popleft()
    if c == '(':
        v = e(l)
        assert l.popleft() == ')'
        return v
    else:
        return int(c)

def e(l):
    assert len(l) > 0, l
    a = n(l)
    while len(l) > 0 and l[0] != ')':
        assert l.popleft() == ' '
        op = l.popleft()
        assert l.popleft() == ' '
        b = n(l)
        if op == '*':
            a*=b
        elif op == '+':
            a+=b
        else:
            assert False, op
    return a

with open('/tmp/input.txt') as f:
    print(sum([e(deque(l.strip())) for l in f]))
