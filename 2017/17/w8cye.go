package main

import "slices"

func part1(data int) int {
	ring := make([]int, 1, 2018)
	pos := 0

	for size := 1; size < 2018; size++ {
		pos = (pos+data)%size + 1
		ring = slices.Insert(ring, pos, size)
	}

	return ring[(pos+1)%len(ring)]
}

func part2(data int) int {
	pos := 0
	result := 0

	for size := 1; size <= 50_000_000; size++ {
		pos = (pos+data)%size + 1
		if pos == 1 {
			result = size
		}
	}

	return result
}
