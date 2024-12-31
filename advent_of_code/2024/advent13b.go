// Mandatory comment
package main

import (
	"bufio"
	"fmt"
	"math/big"
	"os"
	"regexp"
	"strconv"
)

func main() {
	f, err := os.Open("/tmp/advent13.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	sc := bufio.NewScanner(f)
	rnum := regexp.MustCompile(`\d+`)
	ps := [][]int64{}
	p := []int64{}
	for sc.Scan() {
		l := sc.Text()
		if len(l) == 0 {
			ps = append(ps, p)
			p = []int64{}
			continue
		}
		for _, s := range rnum.FindAllString(sc.Text(), 2) {
			i, _ := strconv.Atoi(s)
			p = append(p, int64(i))
		}
	}
	if len(p) > 0 {
		ps = append(ps, p)
	}
	// fmt.Println(ps)

	var total int64 = 0
	for _, p := range ps {
		p[4] += 10000000000000
		p[5] += 10000000000000
		bnum := big.NewRat(p[4], p[0])
		bnum = bnum.Sub(bnum, big.NewRat(p[5], p[1]))
		bden := big.NewRat(p[2], p[0])
		bden = bden.Sub(bden, big.NewRat(p[3], p[1]))
		b := bnum.Quo(bnum, bden)
		a := big.NewRat(0, 1)
		a.Mul(b, big.NewRat(p[2], p[0]))
		a.Neg(a)
		a.Add(a, big.NewRat(p[4], p[0]))
		if a.IsInt() && b.IsInt() {
			mincost := a.Num().Int64()*3 + b.Num().Int64()
			total += mincost
		}
	}
	fmt.Println(total)
}
