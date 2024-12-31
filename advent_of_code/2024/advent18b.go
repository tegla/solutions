// Mandatory comment
package main

import (
	"bufio"
	"fmt"
	"iter"
	"math"
	"os"
	"regexp"
	"strconv"
)

func getNumsFromLine(line string) []int {
	rnum := regexp.MustCompile(`\d+`)
	nums := []int{}
	for _, m := range rnum.FindAllString(line, -1) {
		n, err := strconv.Atoi(m)
		if err != nil {
			panic(err)
		}
		nums = append(nums, n)
	}
	return nums
}

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
		for _, dir := range dirs {
			p := prev.p.add(dir)
			if !s.m.outside(p) && s.m.get(p) == '.' {
				if !yield(state{p}, 1) {
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
	f, err := os.Open("/tmp/advent18.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	flakes := [][]int{}
	sc := bufio.NewScanner(f)
	for sc.Scan() {
		flakes = append(flakes, getNumsFromLine(sc.Text()))
	}

	var s searcher
	s.Init()
	for range 71 {
		s.m.m = append(s.m.m, make([]byte, 71))
	}
	s.m.clean('.')

	for _, flake := range flakes[:1024] {
		s.m.set(pt{r: flake[1], c: flake[0]}, '#')
	}
	s.m.dump()
	s.costs[state{pt{r: 0, c: 0}}] = 0
	s.fill()
	for p := range s.m.Pts() {
		s.m.set(p, 'O')
	}
	fmt.Println(s.costs[state{pt{r: 70, c: 70}}])
}
