// Day 22 - Part 1:    5xxx - Execution time:     359µs
// Day 22 - Part 2: 2511xxx - Execution time:  415930µs

package main

import (
	"bufio"
	"fmt"
	"os"
	"time"
)

type Vector2D struct {
	row, col int
}

type Flag int

const (
	clean Flag = iota
	weakened
	infected
	flagged
)

type Sporifica struct {
	position  Vector2D
	direction Vector2D
	infected  int
	points    map[Vector2D]Flag
}

func (s *Sporifica) move() {
	s.position.row += s.direction.row
	s.position.col += s.direction.col
}

func (s *Sporifica) turnRight() {
	s.direction.row, s.direction.col = s.direction.col, -s.direction.row
}

func (s *Sporifica) turnLeft() {
	s.direction.row, s.direction.col = -s.direction.col, s.direction.row
}

func (s *Sporifica) turnAround() {
	s.direction.row, s.direction.col = -s.direction.row, -s.direction.col
}

func (s *Sporifica) part1() int {
	for range 10_000 {
		switch s.points[s.position] {
		case clean:
			s.turnLeft()
			s.points[s.position] = infected
			s.infected++
		case infected:
			s.turnRight()
			// s.points[s.position] = clean
			// Deleting point is more efficient for small maps
			delete(s.points, s.position)
		}
		s.move()
	}
	return s.infected
}

func (s *Sporifica) part2() int {
	// Sizing map to avoid reallocation is worth maybe 10% speedup
	// s.points = make(map[Vector2D]Flag, 1<<19)
	for range 10_000_000 {
		switch s.points[s.position] {
		case clean:
			s.turnLeft()
			s.points[s.position] = weakened
		case weakened:
			s.points[s.position] = infected
			s.infected++
		case infected:
			s.turnRight()
			s.points[s.position] = flagged
		case flagged:
			s.turnAround()
			s.points[s.position] = clean
			// Flag clean is more efficient for large maps
			// delete(s.points, s.position)
		}
		s.move()
	}
	fmt.Println(len(s.points))
	return s.infected
}

func loadFromFile(filename string) ([]string, error) {
	file, err := os.Open(filename)
	if err != nil {
		fmt.Println("Error opening file:", err)
		return nil, err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	lines := make([]string, 0, 25)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
		return nil, err
	}

	return lines, nil
}

func (s *Sporifica) parseInput(lines []string) {
	for row := range lines {
		for col := range lines[row] {
			if lines[row][col] == '#' {
				s.points[Vector2D{row, col}] = infected
			}
		}
	}
}

func NewSporifica(lines []string) *Sporifica {
	s := &Sporifica{
		position:  Vector2D{len(lines) / 2, len(lines[0]) / 2},
		direction: Vector2D{-1, 0},
		points:    make(map[Vector2D]Flag),
	}
	s.parseInput(lines)
	return s
}

func main() {
	lines, err := loadFromFile("input.txt")
	if err != nil {
		os.Exit(1)
	}

	t := time.Now()
	p1 := NewSporifica(lines).part1()
	t1 := time.Since(t)
	fmt.Printf("Day 22 - Part 1: %7d - Execution time: %15s\n", p1, t1.String())

	t = time.Now()
	p2 := NewSporifica(lines).part2()
	t2 := time.Since(t)
	fmt.Printf("Day 22 - Part 2: %7d - Execution time: %15s\n", p2, t2.String())

}
