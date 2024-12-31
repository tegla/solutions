// Package advent01a solves Advent of Code 2024, puzzle 2, part a.
package main

import (
	"bufio"
	"fmt"
	"os"
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
		s := safe(l)
		if s {
			count++
		}
		fmt.Println(s, l)
	}
	fmt.Println(count)
}
