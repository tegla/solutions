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

func dumpDeps(b [][]bool) {
	fmt.Println("deps:")
	for i := range b {
		ds := []int{}
		for j := range b[i] {
			if b[i][j] {
				ds = append(ds, j)
			}
		}
		if len(ds) > 0 {
			fmt.Println(i, ds)
		}
	}
}

func main() {
	f, err := os.Open("/tmp/advent05.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()
	before := [][]int{}

	sc := bufio.NewScanner(f)
	for sc.Scan() {
		if sc.Text() == "" {
			break
		}
		before = append(before, atois(strings.Split(sc.Text(), "|")))
	}
	fmt.Println(before)
	d := make([][]bool, 100)
	for i := range d {
		d[i] = make([]bool, 100)
	}
	for _, p := range before {
		d[p[1]][p[0]] = true
	}
	dumpDeps(d)
	total := 0
	for sc.Scan() {
		update := atois(strings.Split(sc.Text(), ","))
		broken := false
		for i := 0; i < len(update)-1; i++ {
			for j := i + 1; j < len(update); j++ {
				if d[update[i]][update[j]] {
					broken = true
					break
				}
			}
		}
		fmt.Println(broken, update)
		if !broken {
			total += update[len(update)/2]
		}
	}
	fmt.Println(total)
}
