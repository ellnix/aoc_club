package day08

import (
	"strconv"
	"strings"
)

// Create a map which links various inc and dec to their expected functions
// func signature can be func(int, int) int , while keys can be string
// then make another map which links operations like !=, ==, >=, <=, <, >
// to their respective functions. the key type would be string and the value type
// would be func(int, int) bool.

var ops = map[string]func(int, int) int{ // nolint:gochecknoglobals // its fine
	"inc": func(x, y int) int { return x + y },
	"dec": func(x, y int) int { return x - y },
}

var pred = map[string]func(int, int) bool{ // nolint:gochecknoglobals // its fine
	">":  func(x, y int) bool { return x > y },
	"<":  func(x, y int) bool { return x < y },
	">=": func(x, y int) bool { return x >= y },
	"<=": func(x, y int) bool { return x <= y },
	"==": func(x, y int) bool { return x == y },
	"!=": func(x, y int) bool { return x != y },
}

func solve(input []string, mem func(string, ...int) int) error {
	for _, inst := range input {
		sInst := strings.Fields(inst)
		pNum, err := strconv.Atoi(sInst[6])
		if err != nil {
			return err
		}
		if !pred[sInst[5]](mem(sInst[4]), pNum) {
			continue
		}
		oNum, err := strconv.Atoi(sInst[2])
		if err != nil {
			return err
		}
		mem(sInst[0], ops[sInst[1]](mem(sInst[0]), oNum))
	}
	return nil
}

func SolveOne(input []string) (int, error) {
	mem := make(map[string]int)
	memFunc := func(s string, i ...int) int {
		if len(i) == 0 {
			return mem[s]
		}
		mem[s] = i[0]
		return 0
	}
	err := solve(input, memFunc)
	if err != nil {
		return 0, err
	}
	var mNum int
	for _, num := range mem {
		mNum = max(mNum, num)
	}
	return mNum, nil
}

func SolveTwo(input []string) (int, error) {
	mem := make(map[string]int)
	var mNum int
	memFunc := func(s string, i ...int) int {
		if len(i) == 0 {
			return mem[s]
		}
		mem[s] = i[0]
		mNum = max(mNum, i[0])
		return 0
	}
	err := solve(input, memFunc)
	return mNum, err
}
