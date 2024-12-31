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

func fillIndirectDepsRec(b [][]bool, d []bool, i int) {
	if d[i] {
		return
	}
	for j := range b[i] {
		if !b[i][j] {
			continue
		}
		fillIndirectDepsRec(b, d, j)
		for k := range b[j] {
			b[i][k] = b[i][k] || b[j][k]
		}
	}
	d[i] = true
}

// b[i][j] == true means that i depends on j
func fillIndirectDeps(b [][]bool) {
	d := make([]bool, len(b))
	for i := range b {
		fillIndirectDepsRec(b, d, i)
	}
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
	// fmt.Println(before)
	total := 0
	for sc.Scan() {
		update := atois(strings.Split(sc.Text(), ","))
		u := make([]bool, 100)
		for _, i := range update {
			u[i] = true
		}
		d := make([][]bool, 100)
		for i := range d {
			d[i] = make([]bool, 100)
		}
		for _, p := range before {
			if u[p[0]] && u[p[1]] {
				d[p[1]][p[0]] = true
			}
		}
		fillIndirectDeps(d)
		// dumpDeps(d)
		broken := false
		for i := 0; i < len(update)-1; i++ {
			for j := i + 1; j < len(update); j++ {
				if d[update[i]][update[j]] {
					broken = true
					break
				}
			}
		}
		if broken {
			fmt.Println(update)
			// Bubble sort the update.
			for i := 0; i < len(update)-1; i++ {
				for j := i + 1; j < len(update); j++ {
					if d[update[i]][update[j]] {
						update[i], update[j] = update[j], update[i]
					}
				}
			}
			fmt.Println(update)
			total += update[len(update)/2]
		}
	}
	fmt.Println(total)
}
