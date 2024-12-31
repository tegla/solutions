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

func main() {
	f, err := os.Open("/tmp/advent10.txt")
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

	r := make(map[pt]map[pt]struct{})
	for p := range m.Pts() {
		r[p] = make(map[pt]struct{})
		if m.get(p) == '9' {
			r[p][p] = struct{}{}
		}
	}

	for {
		changed := false
		for p := range m.Pts() {
			for _, d := range dirs {
				q := p.add(d)
				if m.outside(q) {
					continue
				}
				if m.get(q) != m.get(p)+1 {
					continue
				}
				for rr := range r[q] {
					_, alreadyexists := r[p][rr]
					if !alreadyexists {
						r[p][rr] = struct{}{}
						changed = true
					}
				}
			}
		}
		if !changed {
			break
		}
	}

	total := 0
	for p := range m.Pts() {
		if m.get(p) != '0' {
			continue
		}
		rr := r[p]
		fmt.Println(p, len(rr), rr)
		total += len(rr)
	}
	fmt.Println(total)
}
