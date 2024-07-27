package day12

import (
	"strconv"
	"strings"
)

func SolveOne(input []string) (int, error) {
	nodeMap, err := parseInput(input)
	if err != nil {
		return 0, err
	}
	mem := make([]bool, len(nodeMap))
	return countNodes(0, nodeMap, mem), nil
}

func SolveTwo(input []string) (int, error) {
	nodeMap, err := parseInput(input)
	if err != nil {
		return 0, err
	}
	mem := make([]bool, len(nodeMap))
	var count int
	for num := range nodeMap {
		if mem[num] {
			continue
		}
		_ = countNodes(num, nodeMap, mem)
		count++
	}
	return count, nil
}

func countNodes(num int, nodeMap [][]int, mem []bool) int {
	mem[num] = true
	var count int
	for _, val := range nodeMap[num] {
		if mem[val] {
			continue
		}
		count += countNodes(val, nodeMap, mem)
	}
	return count + 1
}

func parseInput(input []string) ([][]int, error) {
	output := make([][]int, len(input))
	for _, item := range input {
		splitItem := strings.Split(item, " <-> ")
		splitValues := strings.Split(splitItem[1], ", ")
		parent, err := strconv.Atoi(splitItem[0])
		if err != nil {
			return nil, err
		}
		for _, numChar := range splitValues {
			num, err := strconv.Atoi(numChar) //nolint: govet // its fine to declare 'err' here.
			if err != nil {
				return nil, err
			}
			output[parent] = append(output[parent], num)
		}
	}
	return output, nil
}
