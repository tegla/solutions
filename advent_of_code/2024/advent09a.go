// Mandatory comment
package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	f, err := os.Open("/tmp/advent09.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	sc := bufio.NewScanner(f)
	sc.Scan()
	input := []byte(sc.Text())

	exp := []int{}

	id := 0
	fill := true
	for _, c := range input {
		if fill {
			for range c - '0' {
				exp = append(exp, id)
			}
			id++
		} else {
			for range c - '0' {
				exp = append(exp, -1)
			}
		}
		fill = !fill
	}

	p0 := 0
	p1 := len(exp) - 1

	for p0 < p1 {
		if exp[p0] != -1 {
			p0++
			continue
		}
		if exp[p1] == -1 {
			p1--
			continue
		}
		exp[p0] = exp[p1]
		exp[p1] = -1
		p0++
		p1--
	}

	total := 0
	for i, c := range exp {
		if c == -1 {
			continue
		}
		total += c * i
	}
	fmt.Println(total)
}
