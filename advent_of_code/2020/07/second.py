import input
from functools import lru_cache

bs = dict(input.bs)

@lru_cache(maxsize=None)
def holds(b):
    return sum([n*(holds(o)+1) for n,o in bs[b]])

print(holds('shiny gold'))
