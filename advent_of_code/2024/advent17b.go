// Mandatory comment
package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"regexp"
	"strconv"
)

type machine struct {
	a, b, c int
	ops     []int
	ip      int
	output  []int
}

func getNums(sc *bufio.Scanner) []int {
	rnum := regexp.MustCompile(`\d+`)
	nums := []int{}
	if !sc.Scan() {
		panic("no input")
	}
	for _, m := range rnum.FindAllString(sc.Text(), -1) {
		n, err := strconv.Atoi(m)
		if err != nil {
			panic(err)
		}
		nums = append(nums, n)
	}
	return nums
}

func (m *machine) Init(sc *bufio.Scanner) {
	m.a = getNums(sc)[0]
	m.b = getNums(sc)[0]
	m.c = getNums(sc)[0]
	m.ip = 0
	sc.Scan()
	m.ops = getNums(sc)
	m.output = []int{}
}

func (m *machine) Clone() machine {
	ret := machine{}
	ret.a = m.a
	ret.b = m.b
	ret.c = m.c
	ret.ip = m.ip
	ret.ops = make([]int, len(m.ops))
	copy(ret.ops, m.ops)
	ret.output = make([]int, len(m.output))
	copy(ret.output, m.output)
	return ret
}

func (m *machine) dumpState() {
	fmt.Println("A:", m.a)
	fmt.Println("B:", m.b)
	fmt.Println("C:", m.c)
	fmt.Println("IP:", m.ip)
	fmt.Print("Program: ")
	for _, o := range m.ops {
		fmt.Print(o)
		fmt.Print(",")
	}
	fmt.Println()
	fmt.Print("Output:  ")
	for _, o := range m.output {
		fmt.Print(o)
		fmt.Print(",")
	}
	fmt.Println()
}

func (m *machine) getCombo(combo int) (int, string) {
	if combo <= 3 {
		return combo, strconv.Itoa(combo)
	}
	if combo == 4 {
		return m.a, "A"
	}
	if combo == 5 {
		return m.b, "B"
	}
	if combo == 6 {
		return m.c, "C"
	}
	return combo, ""
}

func (m *machine) stepReal(ip int, dryRun bool, printOp bool) {
	opcode, literal := m.ops[ip], m.ops[ip+1]
	combo, comboStr := m.getCombo(literal)
	if printOp {
		fmt.Print(ip, ": ")
	}
	switch opcode {
	case 0:
		if !dryRun {
			m.a /= (1 << combo)
		}
		if printOp {
			fmt.Println("adv", comboStr)
		}
	case 1:
		if !dryRun {
			m.b ^= literal
		}
		if printOp {
			fmt.Println("bxl", literal)
		}
	case 2:
		val, valStr := m.getCombo(literal)
		if !dryRun {
			m.b = val % 8
		}
		if printOp {
			fmt.Println("bst", valStr)
		}
	case 3:
		if !dryRun {
			if m.a != 0 {
				m.ip = literal - 2
			}
		}
		if printOp {
			fmt.Println("jnz", literal)
		}
	case 4:
		if !dryRun {
			m.b ^= m.c
		}
		if printOp {
			fmt.Println("bxc")
		}
	case 5:
		if !dryRun {
			m.output = append(m.output, combo%8)
		}
		if printOp {
			fmt.Println("out", comboStr)
		}
	case 6:
		if !dryRun {
			m.b = m.a / (1 << combo)
		}
		if printOp {
			fmt.Println("bdv", comboStr)
		}
	case 7:
		if !dryRun {
			m.c = m.a / (1 << combo)
		}
		if printOp {
			fmt.Println("cdv", comboStr)
		}
	}
	if !dryRun {
		m.ip += 2
	}
}

func (m *machine) run() {
	for m.ip = 0; m.ip < len(m.ops); {
		m.stepReal(m.ip, false, false)
		// m.dumpState()
		// fmt.Println()
	}
}

func (m *machine) dumpCode() {
	for ip := 0; ip < len(m.ops); ip += 2 {
		m.stepReal(ip, true, true)
	}
}

func extendAforOut(mOrig *machine, startA int, out int) []int {
	as := []int{}
	for a := 0; a < 8; a++ {
		m := mOrig.Clone()
		m.a = startA<<3 + a
		for range 7 {
			m.stepReal(m.ip, false, false)
		}
		if len(m.output) != 1 {
			panic("not 1")
		}
		if m.output[0] == out {
			as = append(as, startA<<3+a)
		}
	}
	return as
}

func main() {
	f, err := os.Open("/tmp/advent17.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	var m machine
	m.Init(bufio.NewScanner(f))
	m.dumpState()
	fmt.Println()
	m.dumpCode()
	fmt.Println()

	as := map[int]struct{}{0: struct{}{}}
	for i := len(m.ops) - 1; i >= 0; i-- {
		tmp := map[int]struct{}{}
		for a := range as {
			for _, newA := range extendAforOut(&m, a, m.ops[i]) {
				tmp[newA] = struct{}{}
			}
		}
		fmt.Println("for ", m.ops[i:], ": ", tmp)
		as = tmp
	}

	minA := math.MaxInt
	for a := range as {
		if a < minA {
			minA = a
		}
	}
	fmt.Println("minA:", minA)

	// Sanity check:
	m.a = minA
	m.run()
	m.dumpState()
}
