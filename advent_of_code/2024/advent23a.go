package main

import (
	"bufio"
	"fmt"
	"os"
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

	fmt.Println(edges)
	fmt.Println(nodes)

	total := 0

	for n1 := range nodes {
		for n2 := range edges[n1] {
			for n3 := range edges[n2] {
				if n3 == n1 {
					continue
				}
				if !edges[n3].Contains(n1) {
					continue
				}
				if n1[0] == 't' || n2[0] == 't' || n3[0] == 't' {
					total++
				}
			}
		}
	}
	fmt.Println(total / 6)
}
