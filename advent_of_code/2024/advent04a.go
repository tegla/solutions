// Mandatory comment
package main

import (
	"bufio"
	"fmt"
	"os"
)

func dump(m []string) {
	for _, s := range m {
		fmt.Println(s)
	}
}

func has(m []string, patt string, r int, c int, rd int, cd int) bool {
	if len(patt) == 0 {
		return true
	}
	if r < 0 || r >= len(m) {
		return false
	}
	mr := m[r]
	if c < 0 || c >= len(mr) {
		return false
	}
	if mr[c] != patt[0] {
		return false
	}
	return has(m, patt[1:], r+rd, c+cd, rd, cd)
}

var dirs = [][]int{
	{1, 0},
	{0, 1},
	{-1, 0},
	{0, -1},
	{1, 1},
	{-1, 1},
	{1, -1},
	{-1, -1},
}

func main() {
	f, err := os.Open("/tmp/advent04.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()
	m := []string{}
	sc := bufio.NewScanner(f)
	for sc.Scan() {
		m = append(m, sc.Text())
	}
	dump(m)

	total := 0
	for r, mr := range m {
		for c := range mr {
			for _, d := range dirs {
				if has(m, "XMAS", r, c, d[0], d[1]) {
					fmt.Println(r, c)
					total++
				}
			}
		}
	}
	fmt.Println(total)
}
