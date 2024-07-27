package day10

import (
	"fmt"
	"strconv"
	"strings"
)

func SolveOne(input string, limit int) (any, error) {
	data := genSeq(limit)
	seq, err := toSplitInt(input, ",")
	if err != nil {
		return nil, err
	}
	tieKnots(data, seq, 1)
	return data[0] * data[1], nil
}

func SolveTwo(input string, limit int) (any, error) {
	data := genSeq(limit)
	seq := append(toASCIISlc(input), 17, 31, 73, 47, 23)
	tieKnots(data, seq, 64)
	var output strings.Builder
	for _, num := range xorReduce(data) {
		output.WriteString(fmt.Sprintf("%02x", num))
	}
	return output.String(), nil
}

func tieKnots(data, seq []int, times int) {
	var skip, start int
	for range times {
		for _, num := range seq {
			modifyArray(data, start, num)
			start = modulo(start+num+skip, len(data))
			skip++
		}
	}
}

func xorReduce(input []int) []int {
	output := make([]int, len(input)/16)
	var p int
	for idx := range output {
		var val int
		for _, num := range input[p : p+16] {
			val ^= num
		}
		output[idx] = val
		p += 16
	}
	return output
}

func modifyArray(data []int, start, limit int) {
	for i := range limit / 2 {
		j, k := modulo(start+i, len(data)), modulo(start+(limit-1)-i, len(data))
		data[j], data[k] = data[k], data[j]
	}
}

func genSeq(arrayLen int) []int {
	out := make([]int, arrayLen)
	for i := range arrayLen {
		out[i] = i
	}
	return out
}

func toASCIISlc(input string) []int {
	var output []int
	for _, char := range []byte(input) {
		output = append(output, int(char))
	}
	return output
}

func toSplitInt(input, sep string) ([]int, error) {
	var output []int
	for _, numChar := range strings.Split(input, sep) {
		num, err := strconv.Atoi(numChar)
		if err != nil {
			return nil, err
		}
		output = append(output, num)
	}
	return output, nil
}

// from: https://overflow.adminforge.de/questions/3417183/modulo-of-negative-numbers
func modulo(num, limit int) int {
	return ((num % limit) + limit) % limit
}
