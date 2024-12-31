// Package advent01a solves Advent of Code 2024, puzzle 2, part b.
package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func safe(l []int) bool {
	increasing := l[0] < l[1]
	for i := 0; i < len(l)-1; i++ {
		d := l[i+1] - l[i]
		if d < 0 {
			d = -d
		}
		if d < 1 {
			return false
		}
		if d > 3 {
			return false
		}
		if increasing {
			if l[i] > l[i+1] {
				return false
			}
		} else {
			if l[i] < l[i+1] {
				return false
			}
		}
	}
	return true
}

func lazysafe(l []int) bool {
	if safe(l) {
		return true
	}
	for i := 0; i < len(l); i++ {
		ll := slices.Concat(l[:i], l[i+1:])
		if safe(ll) {
			return true
		}
	}
	return false
}

func main() {
	f, err := os.Open("/tmp/advent02.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()
	sc := bufio.NewScanner(f)
	m := [][]int{}
	for sc.Scan() {
		fields := strings.Fields(sc.Text())
		m1 := []int{}
		for _, f := range fields {
			i, err := strconv.Atoi(f)
			if err != nil {
				panic(err)
			}
			m1 = append(m1, i)
		}
		m = append(m, m1)
	}
	if err := sc.Err(); err != nil {
		panic(err)
	}
	count := 0
	for _, l := range m {
		s := lazysafe(l)
		if s {
			count++
		}
		fmt.Println(s, l)
	}
	fmt.Println(count)
}
