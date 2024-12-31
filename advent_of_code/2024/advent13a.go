// Mandatory comment
package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
)

func main() {
	f, err := os.Open("/tmp/advent13.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	sc := bufio.NewScanner(f)
	rnum := regexp.MustCompile(`\d+`)
	ps := [][]int{}
	p := []int{}
	for sc.Scan() {
		l := sc.Text()
		if len(l) == 0 {
			ps = append(ps, p)
			p = []int{}
			continue
		}
		for _, s := range rnum.FindAllString(sc.Text(), 2) {
			i, _ := strconv.Atoi(s)
			p = append(p, i)
		}
	}
	if len(p) > 0 {
		ps = append(ps, p)
	}
	// fmt.Println(ps)

	total := 0
	for _, p := range ps {
		mincost := math.MaxInt
		for a := range 100 {
			for b := range 100 {
				x := p[0]*a + p[2]*b
				y := p[1]*a + p[3]*b
				if x == p[4] && y == p[5] {
					cost := 3*a + b
					if cost < mincost {
						mincost = cost
					}
				}
			}
		}
		fmt.Println(p, mincost)
		if mincost != math.MaxInt {
			total += mincost
		}
	}
	fmt.Println(total)
}
