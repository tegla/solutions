package main

import (
	"bufio"
	"fmt"
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
		panic(d)
	}
}

var numKeypad = keypad{}
var roboKeypad = keypad{}

type state struct {
	robo1, robo2, num byte
}

func (s state) String() string {
	return string([]byte{s.robo1, s.robo2, s.num})
}

func newState(s string) state {
	return state{
		robo1: s[0],
		robo2: s[1],
		num:   s[2],
	}
}

var badState = newState("XXX")

func (s state) pushButton(d byte) (state, byte) {
	if d == 'A' {
		if s.robo1 == 'A' {
			if s.robo2 == 'A' {
				return s, s.num
			} else {
				s.num = numKeypad.getFor(s.num, s.robo2)
				if s.num == ' ' {
					return badState, ' '
				}
			}
			return s, ' '
		} else {
			s.robo2 = roboKeypad.getFor(s.robo2, s.robo1)
			if s.robo2 == ' ' {
				return badState, ' '
			}
		}
	} else {
		s.robo1 = roboKeypad.getFor(s.robo1, d)
		if s.robo1 == ' ' {
			return badState, ' '
		}
	}
	return s, ' '
}

func shortestPathBetweenButtons(a, b byte) int {
	startState := state{robo1: 'A', robo2: 'A', num: a}
	endState := state{robo1: 'A', robo2: 'A', num: b}
	p := map[state]int{}
	p[startState] = 0
	q := []state{startState}
	for len(q) > 0 {
		st := q[0]
		l := p[st] + 1
		q = q[1:]
		for _, d := range []byte("<>^VA") {
			st2, out := st.pushButton(d)
			if st2 == badState || out != ' ' {
				continue
			}
			l2, found := p[st2]
			if !found || l < l2 {
				p[st2] = l
				q = append(q, st2)
			}
		}
	}
	return p[endState]
}

func shortestPath(s string) int {
	s = "A" + s
	total := 0
	for i := 0; i < len(s)-1; i++ {
		total += shortestPathBetweenButtons(s[i], s[i+1])
		total++
	}
	return total
}

func main() {
	numKeypad.Init(numKeypadLayout)
	roboKeypad.Init(roboKeypadLayout)

	f, err := os.Open("/tmp/advent21.txt")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	total := 0
	sc := bufio.NewScanner(f)
	for sc.Scan() {
		s := sc.Text()
		sp := shortestPath(s)
		num, _ := strconv.Atoi(s[:len(s)-1])
		total += sp * num
	}
	fmt.Println(total)
}
