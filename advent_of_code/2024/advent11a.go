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
	ss := strings.Fields(string(sc.Text()))
	fmt.Println(ss)

	for range 25 {
		n := []string{}
		for _, s := range ss {
			if s == "0" {
				n = append(n, "1")
			} else if len(s)%2 == 0 {
				s1 := s[:len(s)/2]
				s2 := s[len(s)/2:]
				for ; len(s2) > 1 && s2[0] == '0'; s2 = s2[1:] {
				}
				n = append(n, s1, s2)
			} else {
				a := big.Int{}
				b := big.Int{}
				b.SetInt64(2024)
				a.SetString(s, 10)
				a.Mul(&a, &b)
				n = append(n, a.String())
			}
		}
		ss = n
		// fmt.Println(n)
	}
	fmt.Println(len(ss))
}
