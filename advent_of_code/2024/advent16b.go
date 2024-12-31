// Mandatory comment
package main

import (
	"bufio"
	"fmt"
	"iter"
	"math"
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

type state struct {
	p pt
	d int
}

type searcher struct {
	m       mm
	costs   map[state]int
	revPath map[state]map[state]struct{}
}

func (s *searcher) Init() {
	s.m = mm{}
	s.costs = map[state]int{}
	s.revPath = map[state]map[state]struct{}{}
}

func (s *searcher) nextState(prev state) iter.Seq2[state, int] {
	return func(yield func(state, int) bool) {
		if s.m.get(prev.p.add(dirs[prev.d])) == '.' {
			if !yield(state{prev.p.add(dirs[prev.d]), prev.d}, 1) {
				return
			}
		}
		for d, dir := range dirs {
			if (d+prev.d)%2 == 1 {
				if s.m.get(prev.p.add(dir)) == '.' {
					if !yield(state{prev.p.add(dir), d}, 1001) {
						return
					}
				}
			}
		}
	}
}

func (s *searcher) fill() {
	q := []state{}
	for st := range s.costs {
		q = append(q, st)
	}
	for len(q) > 0 {
		st := q[0]
		// fmt.Println("at ", st)
		q = q[1:]
		for next, cost := range s.nextState(st) {
			// fmt.Println("  next: ", next, " cost: ", cost)
			newCost := cost + s.costs[st]
			oldCost, found := s.costs[next]
			if !found {
				oldCost = math.MaxInt
			}
			if newCost <= oldCost {
				if s.revPath[next] == nil {
					s.revPath[next] = map[state]struct{}{}
				}
				if newCost < oldCost {
					s.revPath[next] = map[state]struct{}{}
					s.costs[next] = newCost
					q = append(q, next)
				}
				s.revPath[next][st] = struct{}{}
			}
		}
	}
}

func (s *searcher) cleanRevPath(end state) {
	saved := s.revPath
	s.revPath = map[state]map[state]struct{}{}
	q := []state{end}
	for len(q) > 0 {
		st := q[0]
		q = q[1:]
		if s.revPath[st] != nil {
			continue
		}
		s.revPath[st] = saved[st]
		for prev := range s.revPath[st] {
			q = append(q, prev)
		}
	}
}

func main() {
	f, err := os.Open("/tmp/advent16.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()
	var s searcher
	s.Init()

	sc := bufio.NewScanner(f)
	for sc.Scan() {
		s.m.m = append(s.m.m, []byte(sc.Text()))
	}
	s.m.dump()
	start := pt{len(s.m.m) - 2, 1}
	end := pt{1, len(s.m.m[0]) - 2}
	if s.m.get(start) != 'S' || s.m.get(end) != 'E' {
		panic("bad example")
	}
	s.m.set(end, '.')
	s.m.set(start, '.')
	s.costs[state{start, 1}] = 0
	s.fill()
	minCost := math.MaxInt
	var minEnd state
	for dir := range dirs {
		c, found := s.costs[state{end, dir}]
		if found && c < minCost {
			minCost = c
			minEnd = state{end, dir}
		}
	}
	fmt.Println(minCost)
	fmt.Println(minEnd)
	s.cleanRevPath(minEnd)
	for st := range s.revPath {
		s.m.set(st.p, 'O')
	}
	s.m.dump()
	total := 0
	for p := range s.m.Pts() {
		if s.m.get(p) == 'O' {
			total++
		}
	}
	fmt.Println(total)
}
