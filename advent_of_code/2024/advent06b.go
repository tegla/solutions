// Mandatory comment
package main

import (
	"bufio"
	"fmt"
	"iter"
	"os"
)

func dump(m [][]byte) {
	for r := range m {
		fmt.Println(string(m[r]))
	}
}

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

func (p *pt) add(d pt) {
	p.r += d.r
	p.c += d.c
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

func (m *mm) PtSeq() iter.Seq[pt] {
	return func(yield func(pt) bool) {
		for r := range m.m {
			for c := range m.m[r] {
				yield(pt{r, c})
			}
		}
	}
}

func main() {
	f, err := os.Open("/tmp/advent06.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()
	m := mm{}

	sc := bufio.NewScanner(f)
	for sc.Scan() {
		m.m = append(m.m, []byte(sc.Text()))
	}

	start := pt{0, 0}
	for p := range m.PtSeq() {
		if m.get(p) == '^' {
			start = p
			m.set(p, '.')
		}
	}
	fmt.Println(start)
	m.dump()

	count := 0
	for opt := range m.PtSeq() {
		m := m.clone()
		m.set(opt, '#')
		ms := make([]mm, 4)
		for i := range ms {
			ms[i] = m.clone()
		}
		p := start
		d := 0
		for !m.outside(p) {
			if ms[d].get(p) == 'X' {
				break
			}
			ms[d].set(p, 'X')
			n := p
			n.add(dirs[d])
			if m.get(n) == '#' {
				d = (d + 1) % 4
				continue
			}
			p = n
		}
		if !m.outside(p) {
			fmt.Println(p)
			count++
		}
	}
	fmt.Println(count)
}
