from re import search
import ast


class FlippedEval(ast.NodeVisitor):
    def visit_Constant(self, node: ast.Constant):
        return node.value

    def visit_BinOp(self, node: ast.BinOp):
        left = self.visit(node.left)
        right = self.visit(node.right)
        if isinstance(node.op, ast.Add):
            return left * right
        else:
            return left + right


def v(l):
    flipped = l.replace('+', 'X').replace('*', '+').replace('X', '*')
    e = ast.parse(flipped, mode='eval').body
    return FlippedEval().visit(e)


with open('/tmp/input.txt') as f:
    print(sum([v(l.strip()) for l in f]))
