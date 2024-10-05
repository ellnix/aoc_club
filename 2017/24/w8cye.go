package main

// There are faster ways of doing this, but I wanted to try the iters in the maps package

import (
	"bufio"
	"cmp"
	"fmt"
	"iter"
	"maps"
	"os"
	"strconv"
	"strings"
	"time"
)

type Day24 struct {
	connections    map[int][]int
	componentsUsed map[[2]int]bool
	legthStength   map[int]int

	sum   int
	depth int
}

func main() {
	lines, err := loadFromFile("input.txt")
	if err != nil {
		os.Exit(1)
	}

	t := time.Now()

	d24 := NewDay24(lines)
	fmt.Printf("Day 24 - Part 1: %7d\n", d24.part1())
	fmt.Printf("Day 24 - Part 1: %7d\n", d24.part2())

	t1 := time.Since(t)

	fmt.Printf("Execution time: %15s\n", t1.String())
}

func NewDay24(lines []string) *Day24 {
	d := &Day24{
		connections:    make(map[int][]int),
		componentsUsed: make(map[[2]int]bool),
		legthStength:   make(map[int]int),
	}
	d.parseInput(lines)
	d.find(0)
	return d
}

func (d *Day24) part1() int {
	return maxIter(maps.Values(d.legthStength))
}

func (d *Day24) part2() int {
	i := maxIter(maps.Keys(d.legthStength))
	return d.legthStength[i]
}

func (d *Day24) find(leftPort int) {
	// DFS with backtracking
	d.depth++
	found := false
	for _, rightPort := range d.connections[leftPort] {
		component := [2]int{min(leftPort, rightPort), max(leftPort, rightPort)}
		if d.componentsUsed[component] {
			continue
		}
		found = true

		d.componentsUsed[component] = true
		d.sum += leftPort + rightPort

		d.find(rightPort)

		d.sum -= leftPort + rightPort
		d.componentsUsed[component] = false
	}
	if !found {
		d.legthStength[d.depth] = max(d.legthStength[d.depth], d.sum)
	}
	d.depth--
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

func (d *Day24) parseInput(lines []string) {
	for _, line := range lines {
		values := strings.Split(line, "/")

		leftPort, _ := strconv.Atoi(values[0])
		rightPort, _ := strconv.Atoi(values[1])

		d.connections[leftPort] = append(d.connections[leftPort], rightPort)
		d.connections[rightPort] = append(d.connections[rightPort], leftPort)
	}
}

func maxIter[V cmp.Ordered](i iter.Seq[V]) V {
	var maxValue V
	for v := range i {
		if v > maxValue {
			maxValue = v
		}
	}
	return maxValue
}
