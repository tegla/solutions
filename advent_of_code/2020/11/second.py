class Seating:
    def __init__(self, other=None):
        if other is None:
            with open('/tmp/input.txt') as f:
                self.plan = [list(l.strip()) for l in f]
        else:
            self.plan = list([list(l) for l in other.plan])
        self.rows = len(self.plan)
        self.cols = len(self.plan[0])

    def at(self, row, col):
        if row < 0 or row >= self.rows or col < 0 or col >= self.cols:
            return '.'
        else:
            return self.plan[row][col]

    def see(self, row, col, drow, dcol):
        while True:
            row+=drow
            col+=dcol
            if row < 0 or row >= self.rows or col < 0 or col >= self.cols:
                return '.'
            seen = self.at(row,col)
            if seen != '.':
                return seen

    def surround_occupation(self, row, col):
        directions = [(-1,-1), (-1,0), (-1,1), (0, 1), (1,1),(1,0),(1,-1), (0,-1)]
        return [self.see(row,col, drow, dcol) for drow, dcol in directions].count('#')

    def recompute(self, prev):
        for row in range(0, self.rows):
            for col in range(0, self.cols):
                if prev.at(row, col) == 'L' and prev.surround_occupation(row, col) == 0:
                    self.plan[row][col] = '#'
                elif prev.at(row, col) == '#' and prev.surround_occupation(row, col) >= 5:
                    self.plan[row][col] = 'L'

    def total_occupied(self):
        return sum([l.count('#') for l in self.plan])

    def __eq__(self, other):
        return self.plan == other.plan

    def __repr__(self):
        return '\n'.join([''.join(ls) for ls in self.plan])


prev = Seating()
n = Seating(prev)
while True:
    n.recompute(prev)
    print()
    print(n)
    if prev == n:
        break
    prev = n
    n = Seating(prev)

print(n.total_occupied())