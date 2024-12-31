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

func draw(m [][]bool) {
	for y := range m {
		for x := range m[y] {
			if m[y][x] {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
	fmt.Println()
}

func main() {
	f, err := os.Open("/tmp/advent14.txt")
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

	const wide = 101
	const tall = 103
	m := [][]bool{}
	for range tall {
		m = append(m, make([]bool, wide))
	}
	minvar := math.MaxInt
	mint := 0
	// 101,103 are primes, so after 101*103 iterations, we should be back at time 0
	for t := range wide * tall {
		for y := range m {
			for x := range m[y] {
				m[y][x] = false
			}
		}
		for _, p := range ps {
			x := (p[0] + p[2]*t) % wide
			y := (p[1] + p[3]*t) % tall
			x = (x + wide) % wide
			y = (y + tall) % tall
			m[y][x] = true
		}
		sumx := 0
		countx := 0
		for y := range m {
			for x := range m[y] {
				if m[y][x] {
					sumx += x
					countx++
				}
			}
		}
		mean := sumx / countx
		sumvar := 0
		for y := range m {
			for x := range m[y] {
				if m[y][x] {
					d := x - mean
					sumvar += d * d
				}
			}
		}
		squaredvar := sumvar / countx
		if squaredvar < minvar {
			minvar = squaredvar
			mint = t
			draw(m)
			fmt.Println(t, squaredvar, mean)
		}
	}
	fmt.Println(mint)
}
