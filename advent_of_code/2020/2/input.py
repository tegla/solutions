import re

inputpattern = re.compile('(\d+)-(\d+) ([a-z]): ([a-z]+)')

class L:
    def __init__(self, inputline):
        m = inputpattern.match(inputline)
        self.min = int(m.group(1))
        self.max = int(m.group(2))
        self.letter = m.group(3)
        self.password = m.group(4)

    def __repr__(self):
        return "L('%d-%d %s: %s')" % (self.min, self.max, self.letter, self.password)

with open('/tmp/input.txt') as f:
    ls = list(map(lambda l:L(l), f))

