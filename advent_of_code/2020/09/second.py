with open('/tmp/input.txt') as f:
    ns = [int(l) for l in f]

N=70639851

l=0
r=0
s=0
while l < len(ns):
    if s <  N:
        assert r < len(ns)
        s+=ns[r]
        r+=1
    elif s > N:
        s-=ns[l]
        l+=1
    else:
        ls = ns[l:r]
        print(min(ls)+max(ls))
        break

