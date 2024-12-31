package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var mod = 16777216

func next(s int) int {
	s = (s*64 ^ s) % mod
	s = (s/32 ^ s) % mod
	s = (s*2048 ^ s) % mod
	return s
}

type pattern struct {
	a1, a2, a3, a4 int8
}

func fromBids(b []int) pattern {
	if len(b) != 5 {
		panic("bad input")
	}
	return pattern{
		a1: int8(b[1] - b[0]),
		a2: int8(b[2] - b[1]),
		a3: int8(b[3] - b[2]),
		a4: int8(b[4] - b[3]),
	}
}

func wins(s int) map[pattern]int {
	ret := map[pattern]int{}
	bids := []int{s % 10}
	for range 2000 {
		s = next(s)
		bids = append(bids, s%10)
		if len(bids) == 5 {
			p := fromBids(bids[0:5])
			_, found := ret[p]
			if !found {
				ret[p] = bids[4]
			}
			bids = bids[1:]
		}
	}
	return ret
}

func main() {
	f, err := os.Open("/tmp/advent22.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	totalWins := map[pattern]int{}
	sc := bufio.NewScanner(f)
	for sc.Scan() {
		s, _ := strconv.Atoi(sc.Text())
		w := wins(s)
		for p, c := range w {
			totalWins[p] += c
		}
	}
	bestPrice := 0
	for _, c := range totalWins {
		if c > bestPrice {
			bestPrice = c
		}
	}
	fmt.Println(bestPrice)
}
