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

    def slope(self, rowstep, colstep):
        assert rowstep > 0, rowstep
        assert colstep > 0, colstep
        row = rowstep
        col = colstep
        while row < m.h:
            yield self(row,col)
            row += rowstep
            col += colstep

with open('/tmp/input.txt') as f:
    m = M(f)

