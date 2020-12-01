def input():
    with open('/tmp/input.txt') as f:
        return list(map(lambda l: int(l), f))


def first():
    negs = set()
    for i in input():
        if i in negs:
            print("First Answer: ", i*(2020-i))
        negs.add(2020-i)

def main():
    first()

if __name__ == "__main__":
	main()

