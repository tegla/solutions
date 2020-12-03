class M:
    def __init__(self, ls):
        self.ls = list(l.rstrip() for l in ls)
        self.w = len(self.ls[0])
        self.h = len(self.ls)

    def __call__(self, row, col):
        assert row >= 0, row
        assert row < self.h, row
        assert col >= 0, col
        # explicitly no assert on max col!
        return self.ls[row][col%self.w]

    def slope(self, right, down):
        assert right > 0, right
        assert down > 0, down
        row = down
        col = right
        while row < m.h:
            yield self(row,col)
            row += down
            col += right

with open('/tmp/input.txt') as f:
    m = M(f)

