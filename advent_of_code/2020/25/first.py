
P = 20201227


def tr(sn, ls):
    v = 1
    pv = sn
    while ls > 0:
        if ls % 2 == 1:
            v *= pv
            v %= P
        pv = (pv*pv) % P
        ls //= 2
    return v


PU1 = 2084668
PU2 = 3704642

for ls in range(0, P):
    pu = tr(7, ls)
    if pu == PU1:
        print(tr(PU2, ls))
        break
    if pu == PU2:
        print(tr(PU1, ls))
        break
