package day06

import (
	"fmt"
)

func SolveOne(input []int) int {
	mem := make(map[string]struct{})
	return solve(input, func(s string) (int, bool) {
		_, ok := mem[s]
		if !ok {
			mem[s] = struct{}{}
			return 0, false
		}
		return len(mem), true
	})
}

func SolveTwo(input []int) int {
	mem := make(map[string]int)
	return solve(input, func(s string) (int, bool) {
		num, ok := mem[s]
		if !ok {
			mem[s] = len(mem)
			return 0, false
		}
		return (len(mem) - num), true
	})
}

func solve(input []int, process func(string) (int, bool)) int {
begin:
	num, ok := process(fmt.Sprint(input))
	if ok {
		return num
	}
	var mIdx, maxNum int
	for j, val := range input {
		if val > maxNum {
			maxNum = val
			mIdx = j
		}
	}
	input[mIdx] = 0
	dist, diff := maxNum/len(input), maxNum%len(input)
	for k := 1; k <= len(input); k++ {
		idx := (mIdx + k) % len(input)
		input[idx] += dist
		if diff > 0 {
			input[idx]++
			diff--
		}
	}
	goto begin
}
