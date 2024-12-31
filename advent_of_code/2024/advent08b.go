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

func gcd(a, b int) int {
	for a != 0 {
		a, b = b%a, a
	}
	return b
}

func main() {
	f, err := os.Open("/tmp/advent08.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()
	m := mm{}

	sc := bufio.NewScanner(f)
	for sc.Scan() {
		m.m = append(m.m, []byte(sc.Text()))
	}

	m2 := m.clone()
	for p := range m2.Pts() {
		m2.set(p, '.')
	}

	for p1 := range m.Pts() {
		c := m.get(p1)
		if c == '.' {
			continue
		}
		for p2 := range m.Pts() {
			if p1 == p2 {
				continue
			}
			if m.get(p2) != c {
				continue
			}
			d := p2.sub(p1)
			gcd := gcd(d.r, d.c)
			d.r /= gcd
			d.c /= gcd
			for p3 := p1; !m2.outside(p3); p3 = p3.add(d) {
				m2.set(p3, '#')
			}
			d.r, d.c = -d.r, -d.c
			for p3 := p2; !m2.outside(p3); p3 = p3.add(d) {
				m2.set(p3, '#')
			}
		}
	}
	m2.dump()

	count := 0
	for p := range m2.Pts() {
		if m2.get(p) == '#' {
			count++
		}
	}
	fmt.Println(count)
}
