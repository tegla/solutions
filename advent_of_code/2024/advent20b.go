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
	c int // collisions left
}

type jump struct {
	from pt
	to   pt
}

type searcher struct {
	m       mm
	costs   map[state]int
	revPath map[state]map[state]struct{}
	noJump  map[jump]struct{}
}

func (s *searcher) Init() {
	s.m = mm{}
	s.costs = map[state]int{}
	s.revPath = map[state]map[state]struct{}{}
	s.noJump = map[jump]struct{}{}
}

func (s *searcher) nextState(prev state) iter.Seq2[state, int] {
	return func(yield func(state, int) bool) {
		for _, dir := range dirs {
			p := prev.p.add(dir)
			if !s.m.outside(p) && s.m.get(p) == '.' {
				if !yield(state{p, prev.c}, 1) {
					return
				}
			}
		}
		if prev.c == 0 {
			return
		}
		for x := -20; x <= 20; x++ {
			for y := -20; y <= 20; y++ {
				p2 := prev.p.add(pt{x, y})
				if s.m.outside(p2) {
					continue
				}
				_, nojump := s.noJump[jump{from: prev.p, to: p2}]
				if nojump {
					continue
				}
				if s.m.get(p2) == '#' {
					continue
				}
				var absx, absy int
				if x < 0 {
					absx = -x
				} else {
					absx = x
				}
				if y < 0 {
					absy = -y
				} else {
					absy = y
				}
				distance := absx + absy
				if distance < 2 || distance > 20 {
					continue
				}
				if !yield(state{p2, prev.c - 1}, distance) {
					return
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
		// fmt.Println("at ", st)/
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
	f, err := os.Open("/tmp/advent20.txt")
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

	var start, end pt
	for p := range s.m.Pts() {
		if s.m.get(p) == 'S' {
			start = p
		}
		if s.m.get(p) == 'E' {
			end = p
		}
	}
	fmt.Println(start, "->", end)
	s.m.set(start, '.')
	s.m.set(end, '.')

	for i := 0; ; i++ {
		s.revPath = map[state]map[state]struct{}{}
		s.costs = map[state]int{}
		s.costs[state{start, 1}] = 0
		s.fill()

		_, found1 := s.costs[state{end, 1}]
		if !found1 {
			fmt.Println("no non-cheat paths left")
			break
		}
		_, found0 := s.costs[state{end, 0}]
		if !found0 {
			fmt.Println("no cheats left")
			break
		}
		cheatWin := s.costs[state{end, 1}] - s.costs[state{end, 0}]
		if cheatWin < 100 {
			fmt.Println("no cheats left")
			break
		}
		s.cleanRevPath(state{end, 0})
		for to, from := range s.revPath {
			for from := range from {
				if to.c == 0 && from.c == 1 {
					fmt.Println("cheat for", cheatWin, "at:", from.p, "->", to.p)
					s.noJump[jump{from: from.p, to: to.p}] = struct{}{}
				}
			}
		}
	}
	fmt.Println(len(s.noJump))
}
