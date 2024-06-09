package day04

import (
	"slices"
	"strings"
)

func SolveOne(input []string) int {
	return solve(input, validateNorm)
}

func SolveTwo(input []string) int {
	return solve(input, validateAnag)
}

func solve(input []string, validateFunc func(string) bool) int {
	count := 0
	for _, phrase := range input {
		if validateFunc(phrase) {
			count++
		}
	}
	return count
}

func validateNorm(phrase string) bool {
	mem := make(map[string]struct{})
	for _, word := range strings.Fields(phrase) {
		_, ok := mem[word]
		if ok {
			return false
		}
		mem[word] = struct{}{}
	}
	return true
}

func validateAnag(phrase string) bool {
	mem := make(map[string]struct{})
	for _, word := range strings.Fields(phrase) {
		wordSlc := []byte(word)
		slices.Sort(wordSlc)
		_, ok := mem[string(wordSlc)]
		if ok {
			return false
		}
		mem[string(wordSlc)] = struct{}{}
	}
	return true
}
