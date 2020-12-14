from input import ops
from input import bin

memory = {}

for (m, n, mems) in ops:
    for (addr, v) in mems:
        v&= m
        v|= n
        memory[addr] = v


print(sum(memory.values()))