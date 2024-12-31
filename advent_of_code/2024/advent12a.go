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

var dirs = []pt{
	pt{-1, 0}, // up
	pt{0, 1},  // right
	pt{1, 0},  // down
	pt{0, -1}, // left
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

func moveSame(m *mm, m2 *mm, p pt) {
	m2.set(p, m.get(p))
	m.set(p, '.')
	for _, d := range dirs {
		q := p.add(d)
		if m.outside(q) {
			continue
		}
		if m.get(q) == m2.get(p) {
			moveSame(m, m2, q)
		}
	}
}

func main() {
	f, err := os.Open("/tmp/advent12.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()
	m := mm{}

	sc := bufio.NewScanner(f)
	for sc.Scan() {
		m.m = append(m.m, []byte(sc.Text()))
	}
	m.dump()
	m2 := m.clone()

	total := 0
	for p := range m.Pts() {
		c := m.get(p)
		if c == '.' {
			continue
		}

		m2.clean('.')
		moveSame(&m, &m2, p)
		area := 0
		for p := range m2.Pts() {
			if m2.get(p) != '.' {
				area++
			}
		}
		perimeter := 0
		for p := range m2.Pts() {
			if m2.get(p) != c {
				continue
			}
			for _, d := range dirs {
				q := p.add(d)
				if m2.outside(q) || m2.get(q) == '.' {
					perimeter++
				}
			}
		}
		fmt.Println(string([]byte{c}), area, perimeter, area*perimeter)
		total += area * perimeter
	}
	fmt.Println(total)
}
