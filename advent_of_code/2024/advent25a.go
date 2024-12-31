package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	f, err := os.Open("/tmp/advent25.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	sc := bufio.NewScanner(f)
	keys := [][]int{}
	locks := [][]int{}
	for sc.Scan() {
		l := sc.Text()
		ls := []string{l}
		for sc.Scan() {
			l := sc.Text()
			if l == "" {
				break
			}
			ls = append(ls, sc.Text())
		}
		h := []int{}
		for i := range ls[0] {
			var j int
			for j = range ls {
				if ls[j][i] != ls[0][i] {
					break
				}
			}
			h = append(h, j)
		}
		fmt.Println(ls[0][0:1], h)
		if ls[0][0] == '#' {
			locks = append(locks, h)
		} else {
			keys = append(keys, h)
		}
	}
	total := 0
	for _, l := range locks {
	Keys:
		for _, k := range keys {
			for i := range l {
				if l[i] > k[i] {
					continue Keys
				}
			}
			total++
		}
	}
	fmt.Println(total)
}
