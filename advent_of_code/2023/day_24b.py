from sympy import *

# There's just nothing like sympy in Rust that I could find, sorry.

init_printing(use_unicode=True)

x, y, z, vx, vy, vz, t1, t2, t3 = symbols('x y z vx vy vz t1 t2 t3')

result = symbols('result')

eqs = [
    Eq(result, x + y + z),
]

with open('/tmp/advent_of_code/day_24.txt') as f:
  for t in [t1, t2, t3]:
    l = next(f).strip()
    (pos, speed) = l.split('@')
    pos = sympify(pos.strip().split(','))
    speed = sympify(speed.strip().split(','))
    eqs.append(Eq(t * speed[0] + pos[0], t * vx + x))
    eqs.append(Eq(t * speed[1] + pos[1], t * vy + y))
    eqs.append(Eq(t * speed[2] + pos[2], t * vz + z))


pprint(eqs)
res = solve(
    eqs,
    [result, x, y, z, vx, vy, vz, t1, t2, t3],
)

for r in res:
  r, *_ = r
  pprint(r)
