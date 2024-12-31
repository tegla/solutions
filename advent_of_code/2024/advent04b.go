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

func has(m []string, r int, c int, rd int, cd int) bool {
	if m[r][c] != 'A' {
		return false
	}
	if m[r+rd][c+cd] != 'M' {
		return false
	}
	if m[r-rd][c-cd] != 'S' {
		return false
	}
	return true
}

var dirs = [][]int{
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
	for r := 1; r < len(m)-1; r++ {
		mr := m[r]
		for c := 1; c < len(mr)-1; c++ {
			xs := 0
			for _, d := range dirs {
				if has(m, r, c, d[0], d[1]) {
					xs++
				}
			}
			if xs == 2 {
				fmt.Println(r, c)
				total++
			}
		}
	}
	fmt.Println(total)
}
