import input

valid = 0
for p in input.ps:
    ks = set(p.keys())
    if 'cid' in ks:
        ks.remove('cid')
    if len(ks) == 7:
        valid += 1

print(valid)