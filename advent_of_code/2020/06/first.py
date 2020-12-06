from input import gs
from functools import *
from operator import *

print(sum([len(reduce(or_, g)) for g in gs]))
