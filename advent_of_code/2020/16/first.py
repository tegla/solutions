from input import *

import operator
from functools import reduce

def validforfield(field, num):
    (_, l1, r1, l2, r2) = field
    res = (l1 <= num and num <=r1) or (l2 <= num and num <=r2)
    #print(field, num, res)
    return res

def validforafield(num):
    return reduce(operator.or_, [validforfield(f, num) for f in fields])

c = 0
for t in theirs:
    for num in t:
        if not validforafield(num):
            c+=num

print(c)
