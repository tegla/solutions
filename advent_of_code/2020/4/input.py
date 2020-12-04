def flattened():
    with open('/tmp/input.txt') as f:
        for l in f:
            l = l.rstrip()
            for d in l.split(' '):
                yield d

def ps():
    d = {}
    for l in flattened():
        if l == '':
            yield d
            d = {}
        else:
            k,v = tuple(l.split(':'))
            d[k] = v
    if len(d) > 0:
        yield d

keys = set(['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid', 'cid'])
