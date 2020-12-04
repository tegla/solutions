def input():
    with open('/tmp/input.txt') as f:
        return list(map(lambda l: int(l), f))


def first():
    negs = set()
    for i in input():
        if i in negs:
            print("First Answer: ", i*(2020-i))
        negs.add(2020-i)

def second():
    d = {}
    small = filter(lambda x: x <1010, input())
    for i in range(0, len(small)-1):
        for j in range(i+1, len(small)):
            d[2020-(small[i]+small[j])] = [small[i],small[j]]
    big = filter(lambda x: x>=1010, input())
    for i in big:
        if i in d:
            result = d[i]
            result.append(i)
            print(result)
            mul = 1
            for n in result:
                mul*=n
            print("Second Answer: ", mul)

def main():
    first()
    second()

if __name__ == "__main__":
	main()

