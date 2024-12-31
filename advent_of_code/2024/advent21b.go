package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

var numKeypadLayout = [][]byte{
	[]byte("789"),
	[]byte("456"),
	[]byte("123"),
	[]byte(" 0A"),
}

var roboKeypadLayout = [][]byte{
	[]byte(" ^A"),
	[]byte("<V>"),
}

type keypad struct {
	layout [][]byte

	up, down, left, right []byte
	keys                  []byte
}

func (pad *keypad) Init(keypadLayout [][]byte) {
	pad.layout = keypadLayout
	pad.up = make([]byte, 256)
	for r := range pad.up {
		pad.up[r] = ' '
	}
	pad.down = make([]byte, 256)
	for r := range pad.down {
		pad.down[r] = ' '
	}
	pad.left = make([]byte, 256)
	for r := range pad.left {
		pad.left[r] = ' '
	}
	pad.right = make([]byte, 256)
	for r := range pad.right {
		pad.right[r] = ' '
	}
	for r := range pad.layout {
		for c := range pad.layout[r] {
			if r > 0 {
				pad.up[pad.layout[r][c]] = pad.layout[r-1][c]
			}
			if r < len(pad.layout)-1 {
				pad.down[pad.layout[r][c]] = pad.layout[r+1][c]
			}
			if c > 0 {
				pad.left[pad.layout[r][c]] = pad.layout[r][c-1]
			}
			if c < len(pad.layout[r])-1 {
				pad.right[pad.layout[r][c]] = pad.layout[r][c+1]
			}
		}
	}
	pad.keys = []byte{}
	for _, r := range pad.layout {
		for _, c := range r {
			if c != ' ' {
				pad.keys = append(pad.keys, c)
			}
		}
	}
}

func (pad *keypad) getFor(k byte, d byte) byte {
	switch d {
	case '^':
		return pad.up[k]
	case 'V':
		return pad.down[k]
	case '<':
		return pad.left[k]
	case '>':
		return pad.right[k]
	case 'v':
		return pad.down[k]
	default:
		panic(string([]byte{d}))
	}
}

var numKeypad = keypad{}
var roboKeypad = keypad{}

type route struct {
	from, to byte
}

func (r route) String() string {
	return fmt.Sprintf("%c%c", r.from, r.to)
}

type state struct {
	robok   byte
	targetk byte
}

func (s state) String() string {
	return fmt.Sprintf("%c%c", s.robok, s.targetk)
}

// Minimum number of pushes to get from "AAA1" to "AAA2".
// CostMatrix is the minimum pushes for each of "AA1" -> "AA2".
func getCost(pad keypad, costMatrix map[route]int, from byte, to byte) int {
	p := map[state]int{}
	for _, robok := range roboKeypad.keys {
		for _, targetk := range pad.keys {
			p[state{robok, targetk}] = math.MaxInt
		}
	}
	q := []state{state{robok: 'A', targetk: from}}
	p[q[0]] = 0
	for len(q) > 0 {
		st := q[0]
		q = q[1:]
		for _, newRobok := range roboKeypad.keys {
			st2 := st
			st2.robok = newRobok
			l := p[st] + costMatrix[route{st.robok, st2.robok}]
			if st2.robok != 'A' {
				st2.targetk = pad.getFor(st2.targetk, st2.robok)
				if st2.targetk == ' ' {
					continue
				}
				l++
			}
			if l < p[st2] {
				p[st2] = l
				q = append(q, st2)
			}
		}
	}
	return p[state{robok: 'A', targetk: to}]
}

func getCostMatrix(pad keypad, current map[route]int) map[route]int {
	newMatrix := map[route]int{}
	for _, from := range pad.keys {
		for _, to := range pad.keys {
			newMatrix[route{from, to}] = getCost(pad, current, from, to)
		}
	}
	return newMatrix
}

func shortestPath(costMatrix map[route]int, s string) int {
	s = "A" + s
	total := 0
	for i := 0; i < len(s)-1; i++ {
		total += costMatrix[route{from: s[i], to: s[i+1]}]
		total++
	}
	return total
}

func main() {
	numKeypad.Init(numKeypadLayout)
	roboKeypad.Init(roboKeypadLayout)

	costMatrix := map[route]int{}
	for _, from := range roboKeypad.keys {
		for _, to := range roboKeypad.keys {
			costMatrix[route{from, to}] = 0
		}
	}
	for range 25 {
		costMatrix = getCostMatrix(roboKeypad, costMatrix)
	}
	costMatrix = getCostMatrix(numKeypad, costMatrix)

	f, err := os.Open("/tmp/advent21.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	total := 0
	sc := bufio.NewScanner(f)
	for sc.Scan() {
		s := sc.Text()
		sp := shortestPath(costMatrix, s)
		num, _ := strconv.Atoi(s[:len(s)-1])
		total += sp * num
	}
	fmt.Println(total)
}
