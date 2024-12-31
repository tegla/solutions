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
		hmax := 1
		for range len(ns) - 1 {
			hmax *= 3
		}
		for h := range hmax {
			hl := h
			acc := ns[0]
			for i := 1; i < len(ns); i++ {
				op := hl % 3
				hl /= 3
				if op == 0 {
					acc += ns[i]
				} else if op == 1 {
					acc *= ns[i]
				} else {
					acc, err = strconv.Atoi(strconv.Itoa(acc) + strconv.Itoa(ns[i]))
					if err != nil {
						panic(err)
					}
				}
				if acc > r {
					break
				}
			}
			if acc == r {
				total += acc
				break
			}
		}
	}
	fmt.Println(total)
}
