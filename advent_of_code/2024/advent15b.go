// Mandatory comment
package main

import (
	"bufio"
	"fmt"
	"iter"
	"os"
)

type pt struct {
	r int
	c int
}

var up pt = pt{-1, 0}
var right pt = pt{0, 1}
var down pt = pt{1, 0}
var left pt = pt{0, -1}
var dirs = []pt{up, right, down, left}

var commandStrings = map[byte]int{
	'^': 0,
	'>': 1,
	'v': 2,
	'<': 3,
}

func (p pt) add(d pt) pt {
	p.r += d.r
	p.c += d.c
	return p
}

func (p pt) sub(d pt) pt {
	p.r -= d.r
	p.c -= d.c
	return p
}

type mm struct {
	m [][]byte
}

func (m *mm) outside(p pt) bool { return p.r < 0 || p.r >= len(m.m) || p.c < 0 || p.c >= len(m.m[0]) }

func (m *mm) get(p pt) byte {
	if m.outside(p) {
		return '.'
	}
	return m.m[p.r][p.c]
}

func (m *mm) set(p pt, c byte) {
	if m.outside(p) {
		panic("out of bounds")
	}
	m.m[p.r][p.c] = c
}

func (m *mm) dump() {
	for _, r := range m.m {
		fmt.Println(string(r))
	}
}

func (m *mm) clone() mm {
	ret := mm{}
	ret.m = make([][]byte, len(m.m))
	for r := range m.m {
		ret.m[r] = make([]byte, len(m.m[r]))
		copy(ret.m[r], m.m[r])
	}
	return ret
}

func (m *mm) clean(c byte) {
	for p := range m.Pts() {
		m.set(p, c)
	}
}

func (m *mm) Pts() iter.Seq[pt] {
	return func(yield func(pt) bool) {
		for r := range m.m {
			for c := range m.m[r] {
				if !yield(pt{r, c}) {
					return
				}
			}
		}
	}
}

func canMove(m *mm, pts *map[pt]byte, p pt, d pt) bool {
	_, already := (*pts)[p]
	if already {
		return true
	}
	switch m.get(p) {
	case '@':
		(*pts)[p] = '@'
		return canMove(m, pts, p.add(d), d)
	case '.':
		return true
	case '#':
		return false
	case '[':
		(*pts)[p] = '['
		return canMove(m, pts, p.add(d), d) && canMove(m, pts, p.add(right), d)
	case ']':
		(*pts)[p] = ']'
		return canMove(m, pts, p.add(d), d) && canMove(m, pts, p.add(left), d)
	}
	panic("unreachable")
}

func main() {
	f, err := os.Open("/tmp/advent15.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()
	m := mm{}

	sc := bufio.NewScanner(f)
	for sc.Scan() {
		if len(sc.Text()) == 0 {
			break
		}
		l := []byte{}
		for _, c := range sc.Text() {
			switch c {
			case '@':
				l = append(l, "@."...)
			case '.':
				l = append(l, ".."...)
			case 'O':
				l = append(l, "[]"...)
			case '#':
				l = append(l, "##"...)
			}
		}
		m.m = append(m.m, l)
	}
	m.dump()

	cs := []byte{}
	for sc.Scan() {
		cs = append(cs, []byte(sc.Text())...)
	}
	fmt.Println(string(cs))
	var ego pt
	for p := range m.Pts() {
		if m.get(p) == '@' {
			ego = p
			break
		}
	}
	fmt.Println(ego)

	for _, command := range cs {
		dir := dirs[commandStrings[command]]
		pts := map[pt]byte{}
		if canMove(&m, &pts, ego, dir) {
			for p := range pts {
				m.set(p, '.')
			}
			for p, c := range pts {
				m.set(p.add(dir), c)
			}
			ego = ego.add(dir)
		}
	}

	m.dump()
	total := 0
	for p := range m.Pts() {
		if m.get(p) == '[' {
			total += p.r*100 + p.c
		}
	}
	fmt.Println(total)
}
