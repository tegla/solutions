// Package advent01a solves Advent of Code 2024, puzzle 1, part b.
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("Advent of Code 2024, puzzle 1, part b.")
	f, err := os.Open("/tmp/advent01.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()
	left, right := []int{}, []int{}
	sc := bufio.NewScanner(f)
	for sc.Scan() {
		fields := strings.Fields(sc.Text())
		a, err := strconv.Atoi(fields[0])
		if err != nil {
			panic(err)
		}
		left = append(left, a)
		b, err := strconv.Atoi(fields[1])
		if err != nil {
			panic(err)
		}
		right = append(right, b)
		// fmt.Println(a, b)
	}
	if err := sc.Err(); err != nil {
		panic(err)
	}

	total := 0
	mult := make(map[int]int)
	for _, m := range right {
		mult[m] = mult[m] + 1
	}
	for _, n := range left {
		total += mult[n] * n
	}

	fmt.Println(total)
}
