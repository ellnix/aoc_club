package day14

import (
	"fmt"

	knot "github.com/KasimKaizer/advent_of_code/2017/day_10"
)

const gridSize = 128

var hexToBin = [256]string{
	'0': "0000", '1': "0001", '2': "0010", '3': "0011",
	'4': "0100", '5': "0101", '6': "0110", '7': "0111",
	'8': "1000", '9': "1001", 'a': "1010", 'b': "1011",
	'c': "1100", 'd': "1101", 'e': "1110", 'f': "1111",
	'A': "1010", 'B': "1011", 'C': "1100", 'D': "1101",
	'E': "1110", 'F': "1111",
}

func SolveOne(input string) (int, error) {
	var totalOnes int
	err := toKnotHashBin(input, func(_ int, s string) {
		totalOnes += countOnes(s)
	})
	return totalOnes, err
}

func SolveTwo(input string) (int, error) {
	grid := make([][]bool, gridSize)
	err := toKnotHashBin(input, func(i int, s string) {
		grid[i] = append(grid[i], binToBool(s)...)
	})
	if err != nil {
		return 0, err
	}
	var totalGroups int
	for row := range len(grid) {
		for col := range len(grid[row]) {
			if !grid[row][col] {
				continue
			}
			travelBFS(grid, col, row)
			totalGroups++
		}
	}
	return totalGroups, nil
}

func toKnotHashBin(input string, yeild func(int, string)) error {
	tmpl := fmt.Sprintf("%s-%%d", input)
	for i := range 128 {
		temp := fmt.Sprintf(tmpl, i)
		hash, err := knot.Encrypt(temp)
		if err != nil {
			return err
		}
		for _, char := range []byte(hash) {
			yeild(i, hexToBin[char])
		}
	}
	return nil
}

func countOnes(data string) int {
	var total int
	for _, char := range []byte(data) {
		if char == '1' {
			total++
		}
	}
	return total
}

func binToBool(bin string) []bool {
	out := make([]bool, len(bin))
	for i, char := range []byte(bin) {
		if char == '1' {
			out[i] = true
			continue
		}
		out[i] = false
	}
	return out
}

func travelBFS(grid [][]bool, col, row int) {
	if col < 0 || row < 0 ||
		row >= len(grid) ||
		col >= len(grid[row]) ||
		!grid[row][col] {
		return
	}
	grid[row][col] = false
	travelBFS(grid, col+1, row) // right
	travelBFS(grid, col-1, row) // left
	travelBFS(grid, col, row+1) // down
	travelBFS(grid, col, row-1) // up
}
