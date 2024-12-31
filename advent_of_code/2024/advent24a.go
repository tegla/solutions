package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	f, err := os.Open("/tmp/advent24.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	sc := bufio.NewScanner(f)
	r1 := regexp.MustCompile(`(.*): (\d)`)
	state := map[string]bool{}
	for sc.Scan() {
		l := r1.FindStringSubmatch(sc.Text())
		if len(l) == 0 {
			break
		}
		n, _ := strconv.Atoi(l[2])
		if n == 0 {
			state[l[1]] = false
		} else {
			state[l[1]] = true
		}
	}
	fmt.Println(state)

	r2 := regexp.MustCompile(`(.*) (.*) (.*) -> (.*)`)
	rule := [][]string{}
	for sc.Scan() {
		l := r2.FindStringSubmatch(sc.Text())
		rule = append(rule, l[1:])
	}
	fmt.Println(rule)

	for {
		changed := false
		for _, r := range rule {
			a, af := state[r[0]]
			b, bf := state[r[2]]
			_, cf := state[r[3]]
			if !af || !bf {
				continue
			}
			if cf {
				continue
			}
			switch r[1] {
			case "AND":
				state[r[3]] = a && b
			case "OR":
				state[r[3]] = a || b
			case "XOR":
				state[r[3]] = a != b
			default:
				panic(r[1])
			}
			changed = true
		}

		if !changed {
			break
		}
	}
	fmt.Println(state)

	var result int64
	for k, v := range state {
		if !v || k[0] != 'z' {
			continue
		}
		i, _ := strconv.Atoi(k[1:])
		result |= 1 << i
	}
	fmt.Println(result)
}
