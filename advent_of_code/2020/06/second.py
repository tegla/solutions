from input import gs
from functools import *
from operator import *

print(sum([len(reduce(and_, g)) for g in gs]))
