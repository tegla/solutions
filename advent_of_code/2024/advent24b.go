package main

import (
	"bufio"
	"fmt"
	"math"
	"math/rand"
	"os"
	"regexp"
	"sort"
	"strings"
)

type machine struct {
	rule map[string][]string
}

func (m *machine) Clone() *machine {
	ret := &machine{
		rule: map[string][]string{},
	}
	for k, v := range m.rule {
		ret.rule[k] = v
	}
	return ret
}

func (m *machine) compute(vals map[string]bool, computing map[string]struct{}, node string) (bool, bool) {
	v, found := vals[node]
	if found {
		return v, false
	}
	_, isLoop := computing[node]
	if isLoop {
		return false, true
	}
	computing[node] = struct{}{}
	c := m.rule[node]
	var err bool
	l, err := m.compute(vals, computing, c[0])
	if err {
		return false, true
	}
	r, err := m.compute(vals, computing, c[2])
	if err {
		return false, true
	}
	switch c[1] {
	case "AND":
		v = l && r
	case "OR":
		v = l || r
	case "XOR":
		v = l != r
	default:
		panic(c[1])
	}
	vals[node] = v
	return v, false
}

func zName(i int) string {
	return fmt.Sprintf("z%02d", i)
}

func (m *machine) isDependent(n1 string, n2 string) bool {
	if n1 == n2 {
		return true
	}
	r, found := m.rule[n1]
	if !found {
		return false
	}
	return m.isDependent(r[0], n2) || m.isDependent(r[2], n2)
}

// Brute force Monte Carlo Hamming distance to the rescue!
func (m *machine) hamming() int {
	h := 0
	for range 1000 {
		x := make([]bool, 45)
		y := make([]bool, 45)
		z := make([]bool, 46)
		xb := rand.Intn(1 << 44)
		yb := rand.Intn(1 << 44)
		zb := xb + yb
		for i := range 46 {
			if (xb>>i)&1 != 0 {
				x[i] = true
			}
			if (yb>>i)&1 != 0 {
				y[i] = true
			}
			if (zb>>i)&1 != 0 {
				z[i] = true
			}
		}
		vals := map[string]bool{}
		computing := map[string]struct{}{}
		for i := range 45 {
			vals[fmt.Sprintf("x%02d", i)] = x[i]
			vals[fmt.Sprintf("y%02d", i)] = y[i]
		}
		for i := range 46 {
			zs := zName(i)
			expected := z[i]
			v, err := m.compute(vals, computing, zs)
			if err || v != expected {
				h++
			}
		}
	}
	return h
}

func (m *machine) fixHamming() (*machine, string, string) {
	c := 0
	s := 0
	var best1, best2 string
	besth := math.MaxInt
	for n1 := range m.rule {
		for n2 := range m.rule {
			if n1 >= n2 {
				continue
			}
			if m.isDependent(n1, n2) || m.isDependent(n2, n1) {
				continue
			}
			m2 := m.Clone()
			m2.rule[n1], m2.rule[n2] = m.rule[n2], m.rule[n1]
			h := m2.hamming()
			if h < besth {
				besth = h
				best1 = n1
				best2 = n2
			}
			if c%100 == 0 {
				fmt.Println(c, n1, n2, h)
			}
			s += h
			c++
		}
	}
	fmt.Println(c, s/c, "->", besth, best1, best2)
	m2 := m.Clone()
	m2.rule[best1], m2.rule[best2] = m.rule[best2], m.rule[best1]
	return m2, best1, best2
}

func main() {
	f, err := os.Open("/tmp/advent24.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	sc := bufio.NewScanner(f)
	for sc.Scan() {
		if len(sc.Text()) == 0 {
			break
		}
	}

	r2 := regexp.MustCompile(`(.*) (.*) (.*) -> (.*)`)
	m := new(machine)
	m.rule = map[string][]string{}
	for sc.Scan() {
		l := r2.FindStringSubmatch(sc.Text())
		m.rule[l[4]] = l[1:4]
	}
	// fmt.Println(m.rule)
	// m.getFixed()
	// for range 100 {
	// 	fmt.Println(m.hamming())
	// }
	var res []string
	for range 4 {
		var n1, n2 string
		m, n1, n2 = m.fixHamming()
		res = append(res, n1, n2)

	}
	sort.Strings(res)
	fmt.Println(strings.Join(res, ","))
}
