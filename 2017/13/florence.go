package day13

import (
	"strconv"
	"strings"
)

// the player moves a layer each sec, to determine the postion of the checkers during that time, I could probably utilize maths in some way
// why cause checker also moves 1 block a sec, lets say we have a checker at 7th layer with 4 blocks,
// what we know for sure is that the
// checker has moved 4 times in a level with 4 blocks, and the first cycle has completed, thus 7%3 = 3, why 3? well cause after the first cycle
// checker only moves 3 blocks in each directon. Moreover, check if 7/3 is odd or even, this lets us know of the direction of the checker
// if its even then the checker is moving from top to bottom.
// if its not then checker is moving from bottom to top.

type level struct {
	sec    int
	blocks int
}

func SolveOne(input []string) (int, error) {
	data, err := parse(input)
	if err != nil {
		return 0, err
	}
	return travel(data, 0), nil
}

func SolveTwo(input []string) (int, error) {
	var picSec int
	data, err := parse(input)
	if err != nil {
		return 0, err
	}
	for {
		picSec++
		if travel(data, picSec) == 0 {
			return picSec, nil
		}
	}
}

func travel(input []level, delay int) int {
	var sum int
	for _, l := range input {
		if calculatePos((l.sec+delay), l.blocks) != 0 {
			continue
		}
		sum += ((l.sec + delay) * l.blocks)
	}
	return sum
}

func calculatePos(sec, blocks int) int {
	if sec/blocks < 1 {
		return sec % blocks
	}

	y := sec % (blocks - 1)
	switch x := (sec / (blocks - 1)) % 2; x {
	case 1:
		if y == 0 {
			return blocks - 1
		}
		return (blocks - 1 - y)
	case 0:
		if y == 0 {
			return 0
		}
		return y
	}
	return -1
}

func parse(input []string) ([]level, error) {
	var levels []level
	for _, inst := range input {
		numChars := strings.Split(inst, ": ")
		sec, err := strconv.Atoi(numChars[0])
		if err != nil {
			return nil, err
		}
		blocks, err := strconv.Atoi(numChars[1])
		if err != nil {
			return nil, err
		}
		levels = append(levels, level{sec: sec, blocks: blocks})
	}
	return levels, nil
}
