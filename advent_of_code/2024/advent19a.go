// Mandatory comment
package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func possible(pattern string, towels []string) bool {
	ps := map[int]struct{}{}
	ps[0] = struct{}{}
	updated := true
	for updated == true {
		updated = false
		for p := range ps {
			for _, t := range towels {
				end := len(t) + p
				_, found := ps[end]
				if found {
					continue
				}
				if end <= len(pattern) && pattern[p:end] == t {
					ps[end] = struct{}{}
					updated = true
				}
			}
		}
	}
	_, found := ps[len(pattern)]
	return found
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
		poss := possible(p, towels)
		if poss {
			count++
		}
		fmt.Println(p, poss)
	}
	fmt.Println(count)
}
