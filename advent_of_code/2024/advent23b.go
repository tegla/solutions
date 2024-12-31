package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
)

type set[E comparable] map[E]struct{}

func newSet[E comparable]() set[E] {
	return map[E]struct{}{}
}

func (s set[E]) Add(e E) {
	s[e] = struct{}{}
}

func (s set[E]) Contains(e E) bool {
	_, found := s[e]
	return found
}

func (s set[E]) Remove(e E) {
	delete(s, e)
}

func (s set[E]) Len() int {
	return len(s)
}

func (s set[E]) IsEmpty() bool {
	return len(s) == 0
}

func (s set[E]) AsArray() []E {
	a := make([]E, 0, len(s))
	for e := range s {
		a = append(a, e)
	}
	return a
}
func (s set[E]) String() string {
	return fmt.Sprintf("%v", s.AsArray())
}

type node string

type edge struct {
	from, to node
}

func (e edge) String() string {
	return fmt.Sprintf("%s-%s", e.from, e.to)
}

func comb(n int, r int) int {
	c := 1
	for i := range r {
		c *= (n - i)
	}
	for i := range r {
		c /= (i + 1)
	}
	return c
}

func connecteds(edges map[node]set[node], nodes []node, s []node, max int) int {
	if len(s) == max {
		return 1
	}
	total := 0
Main:
	for _, n := range nodes {
		if len(s) > 0 && s[len(s)-1] >= n {
			continue
		}
		for _, n2 := range s {
			if !edges[n2].Contains(n) {
				continue Main
			}
		}
		total += connecteds(edges, nodes, append(s, n), max)
	}
	return total
}

func main() {
	f, err := os.Open("/tmp/advent23.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	edges := map[node]set[node]{}
	nodes := set[node]{}

	sc := bufio.NewScanner(f)
	for sc.Scan() {
		l := sc.Text()
		a, b := node(l[0:2]), node(l[3:5])
		if edges[a] == nil {
			edges[a] = newSet[node]()
		}
		edges[a].Add(b)
		if edges[b] == nil {
			edges[b] = newSet[node]()
		}
		edges[b].Add(a)
		nodes.Add(node(a))
		nodes.Add(node(b))
	}

	sorted := nodes.AsArray()
	sort.Slice(sorted, func(i, j int) bool {
		return sorted[i] < sorted[j]
	})

	fmt.Println(connecteds(edges, sorted, []node{}, 13))
}
