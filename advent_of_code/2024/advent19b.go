// Mandatory comment
package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func possible(pattern string, towels []string) int {
	ps := make([]int, len(pattern)+1)
	ps[0] = 1
	for p := 1; p < len(ps); p++ {
		for _, t := range towels {
			if len(t) > p {
				continue
			}
			s := p - len(t)
			if pattern[s:p] != t {
				continue
			}
			ps[p] += ps[s]
		}
	}
	return ps[len(pattern)]
}

func main() {
	f, err := os.Open("/tmp/advent19.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	sc := bufio.NewScanner(f)

	towels := []string{}
	sc.Scan()
	for _, s := range strings.Fields(sc.Text()) {
		for ; s[len(s)-1] == ','; s = s[:len(s)-1] {
		}
		towels = append(towels, s)
	}
	sc.Scan()

	patterns := []string{}
	for sc.Scan() {
		patterns = append(patterns, sc.Text())
	}

	count := 0
	for _, p := range patterns {
		c := possible(p, towels)
		count += c
		fmt.Println(p, c)
	}
	fmt.Println(count)
}
