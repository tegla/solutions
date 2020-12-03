import input
import operator
import functools

def trees(right,down):
    return operator.countOf(input.m.slope(right=right,down=down), '#')

values = [
        trees(right=1, down=1),
        trees(right=3, down=1),
        trees(right=5, down=1),
        trees(right=7, down=1),
        trees(right=1, down=2),
        ]

print(values)
print(functools.reduce(operator.mul, values))