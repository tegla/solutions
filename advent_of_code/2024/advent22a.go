package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var mod = 16777216

func next(s int) int {
	s = (s*64 ^ s) % mod
	s = (s/32 ^ s) % mod
	s = (s*2048 ^ s) % mod
	return s
}

func main() {
	f, err := os.Open("/tmp/advent22.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	total := 0
	sc := bufio.NewScanner(f)
	for sc.Scan() {
		s, _ := strconv.Atoi(sc.Text())
		s2 := s
		for range 2000 {
			s2 = next(s2)
		}
		fmt.Println(s, ":", s2)
		total += s2
	}
	fmt.Println(total)
}
