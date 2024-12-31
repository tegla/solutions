// Package advent01a solves Advent of Code 2024, puzzle 1, part a.
package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("Advent of Code 2024, puzzle 1, part a.")
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
	slices.Sort(left)
	slices.Sort(right)
	total := 0
	for i, a := range left {
		b := right[i]
		fmt.Println(a, b)
		diff := b - a
		if diff < 0 {
			diff = -diff
		}
		total += diff
	}
	fmt.Println(total)
}
