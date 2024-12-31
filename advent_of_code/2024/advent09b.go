// Mandatory comment
package main

import (
	"bufio"
	"container/heap"
	"fmt"
	"math"
	"os"
)

type block struct {
	p int
	l int
}

// Straight from the example.
type IntHeap []int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *IntHeap) Push(x any) {
	// Push and Pop use pointer receivers because they modify the slice's length,
	// not just its contents.
	*h = append(*h, x.(int))
}

func (h *IntHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func main() {
	f, err := os.Open("/tmp/advent09.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	sc := bufio.NewScanner(f)
	sc.Scan()
	input := []byte(sc.Text())

	used := []block{}
	free := make([]IntHeap, 10)

	fill := true
	p := 0
	for _, c := range input {
		l := int(c - '0')
		if fill {
			used = append(used, block{p: p, l: l})
		} else {
			free[l] = append(free[l], p)
		}
		p += l
		fill = !fill
	}
	for l := range free {
		heap.Init(&free[l])
	}

	for id := len(used) - 1; id >= 0; id-- {
		b := &used[id]
		p := math.MaxInt
		l := 0
		for i := range free {
			if i < b.l {
				continue
			}
			if len(free[i]) == 0 {
				continue
			}
			fp := free[i][0]
			if b.p < fp {
				continue
			}
			if fp < p {
				p = fp
				l = i
			}
		}
		if l == 0 {
			continue
		}
		p = heap.Pop(&free[l]).(int)
		heap.Push(&free[l-b.l], p+b.l)
		b.p = p
	}

	total := 0
	for id, b := range used {
		for i := b.p; i < b.p+b.l; i++ {
			total += i * id
		}
	}
	fmt.Println(total)
}
