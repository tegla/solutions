
def _gs():
    with open('/tmp/input.txt') as f:
        ss = []
        for l in f:
            l = l.rstrip()
            if l == '':
                yield ss
                ss = []
            else:
                ss.append(set(l))
        yield ss

gs = list(_gs())
