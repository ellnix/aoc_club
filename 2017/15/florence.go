package day15

import (
	"regexp"
	"strconv"
)

type generator struct {
	name     byte
	accumNum int
}

func SolveOne(input []string) (int, error) {
	return solve(input, 40_000_000, func(_ generator) bool { return true })
}

func SolveTwo(input []string) (int, error) {
	return solve(input, 5_000_000, func(g generator) bool {
		switch g.name {
		case 'A':
			return (g.accumNum % 4) == 0
		case 'B':
			return (g.accumNum % 8) == 0
		}
		return false
	})
}

func solve(input []string, iter int, pred func(generator) bool) (int, error) {
	gens, err := parseInput(input)
	genNums := [][]int{'A': {}, 'B': {}}
	if err != nil {
		return 0, err
	}
	var matches int
	for {
		for i := range gens {
			gens[i].accumNum = (gens[i].accumNum * genToFactor(gens[i].name)) % 2147483647
			if pred(gens[i]) {
				genNums[gens[i].name] = append(genNums[gens[i].name], gens[i].accumNum)
			}
		}
		if len(genNums['A']) == 0 || len(genNums['B']) == 0 {
			continue
		}
		matches += isLeast16bEql(genNums['A'][0], genNums['B'][0])
		genNums['A'] = genNums['A'][1:]
		genNums['B'] = genNums['B'][1:]
		iter--
		if iter == 0 {
			return matches, nil
		}
	}
}

var mask = ((1 << 16) - 1)

func isLeast16bEql(num1, num2 int) int {
	if (num1 & mask) == (num2 & mask) {
		return 1
	}
	return 0
}

func genToFactor(char byte) int {
	switch char {
	case 'A':
		return 16807
	case 'B':
		return 48271
	}
	return -1
}

var re = regexp.MustCompile(`Generator ([A-Z]) starts with (\d+)`)

func parseInput(input []string) ([]generator, error) {
	var genCol []generator
	for _, inst := range input {
		matches := re.FindStringSubmatch(inst)
		num, err := strconv.Atoi(matches[2])
		if err != nil {
			return nil, err
		}
		temp := generator{name: matches[1][0], accumNum: num}
		genCol = append(genCol, temp)
	}
	return genCol, nil
}
