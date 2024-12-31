// Mandatory comment
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func atois(ss []string) []int {
	r := []int{}
	for _, s := range ss {
		i, err := strconv.Atoi(s)
		if err != nil {
			panic(err)
		}
		r = append(r, i)
	}
	return r
}

func main() {
	f, err := os.Open("/tmp/advent07.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	total := 0
	sc := bufio.NewScanner(f)
	for sc.Scan() {
		k := strings.Split(sc.Text(), ":")
		r, _ := strconv.Atoi(k[0])
		ns := atois(strings.Fields(k[1]))
		for h := range 1 << (len(ns) - 1) {
			hl := h
			acc := ns[0]
			for i := 1; i < len(ns); i++ {
				if hl&1 == 1 {
					acc += ns[i]
				} else {
					acc *= ns[i]
				}
				hl >>= 1
			}
			if acc == r {
				total += acc
				break
			}
		}
	}
	fmt.Println(total)
}
