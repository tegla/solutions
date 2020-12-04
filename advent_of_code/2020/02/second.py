import input
import operator

def matches(l):
    return (l.password[l.min-1] == l.letter) != (l.password[l.max-1] == l.letter)

print(operator.countOf([matches(l) for l in input.ls], True))
