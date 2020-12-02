import input
import operator

def matches(l):
    c = l.password.count(l.letter)
    return (c>=l.min) and (c<=l.max)

print(operator.countOf([matches(l) for l in input.ls], True))
