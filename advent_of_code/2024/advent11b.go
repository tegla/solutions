// Mandatory comment
package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
	"strings"
)

func main() {
	f, err := os.Open("/tmp/advent11.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	sc := bufio.NewScanner(f)
	sc.Scan()
	ss := map[string]int{}
	for _, s := range strings.Fields(string(sc.Text())) {
		ss[s]++
	}
	fmt.Println(ss)

	for i := range 75 {
		ss2 := map[string]int{}
		for s, c := range ss {
			if s == "0" {
				ss2["1"] += c
			} else if len(s)%2 == 0 {
				s1 := s[:len(s)/2]
				s2 := s[len(s)/2:]
				for ; len(s2) > 1 && s2[0] == '0'; s2 = s2[1:] {
				}
				ss2[s1] += c
				ss2[s2] += c
			} else {
				a := big.Int{}
				b := big.Int{}
				b.SetInt64(2024)
				a.SetString(s, 10)
				a.Mul(&a, &b)
				ss2[a.String()] += c
			}
		}
		ss = ss2
		fmt.Println(i, len(ss))
	}
	total := 0
	for _, c := range ss {
		total += c
	}
	fmt.Println(total)
}
