// Mandatory comment
package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	f, err := os.Open("/tmp/advent03.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()
	r, err := regexp.Compile(`do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)`)
	if err != nil {
		panic(err)
	}

	total := 0
	enabled := true
	sc := bufio.NewScanner(f)
	for sc.Scan() {
		s := sc.Text()
		fmt.Println(s)
		for {
			i := r.FindStringSubmatchIndex(s)
			if len(i) == 0 {
				break
			}
			c := s[i[0]:i[1]]
			fmt.Println(c)
			if c == "do()" {
				enabled = true
			} else if c == "don't()" {
				enabled = false
			} else {
				if enabled {
					a, _ := strconv.Atoi(s[i[2]:i[3]])
					b, _ := strconv.Atoi(s[i[4]:i[5]])
					total += a * b
				}
			}
			s = s[i[1]:]
			//fmt.Println(a, b)
		}
	}
	fmt.Println(total)
}
