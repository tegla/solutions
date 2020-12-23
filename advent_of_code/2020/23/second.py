
input = list(map(int, list("398254716")))

while len(input) < 1000000:
    input.append(len(input)+1)

m=[0]*(len(input)+1)
for i in range(0, len(input)):
    m[input[i]] = input[(i+1)%len(input)]

p = input[0]

for r in range(1,10000000+1):
    ts1 = m[p]
    ts2 = m[ts1]
    ts3 = m[ts2]
    ts4 = m[ts3]
    m[p] = ts4
    d = p
    while True:
        d-=1
        if d == 0:
            d = len(input)
        if d not in {ts1,ts2,ts3}:
            break
    m[ts3] = m[d]
    m[d] = ts1
    p = m[p]

print(m[1]*m[m[1]])
