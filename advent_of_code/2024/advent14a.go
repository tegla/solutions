// Mandatory comment
package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	f, err := os.Open("/tmp/advent14_example.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	sc := bufio.NewScanner(f)
	rnum := regexp.MustCompile(`[\-]?\d+`)
	ps := [][]int{}
	for sc.Scan() {
		p := []int{}
		for _, s := range rnum.FindAllString(sc.Text(), 4) {
			i, _ := strconv.Atoi(s)
			p = append(p, i)
		}
		ps = append(ps, p)
	}

	const wide = 11
	const tall = 7
	quad := make([]int, 4)
	for _, p := range ps {
		fmt.Println(p)
		x := (p[0] + p[2]*100) % wide
		y := (p[1] + p[3]*100) % tall
		x = (x + wide) % wide
		y = (y + tall) % tall
		fmt.Println(x, y)
		if x == wide/2 || y == tall/2 {
			continue
		}
		i := 0
		if x > wide/2 {
			i += 2
		}
		if y > tall/2 {
			i++
		}
		quad[i]++
	}
	fmt.Println(quad)
	mul := 1
	for _, q := range quad {
		mul *= q
	}
	fmt.Println(mul)

}
