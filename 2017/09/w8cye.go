package main

import (
	"fmt"
	"os"
)

const (
	groupOpen    = '{'
	groupClose   = '}'
	garbageOpen  = '<'
	garbageClose = '>'
	ignore       = '!'
)

type stream struct {
	input        []byte
	groupDepth   int
	groupScore   int
	garbageCount int
	index        int
}

func (s *stream) groupOpen() {
	s.groupDepth++
}

func (s *stream) groupClose() {
	s.groupScore += s.groupDepth
	s.groupDepth--
}

func (s *stream) garbage() {
	for s.index++; s.input[s.index] != garbageClose; s.index++ {
		switch s.input[s.index] {
		case ignore:
			s.index++
		default:
			s.garbageCount++
		}
	}
}

func (s *stream) part1() (int, int) {
	for ; s.index < len(s.input); s.index++ {
		switch s.input[s.index] {
		case groupOpen:
			s.groupOpen()
		case groupClose:
			s.groupClose()
		case garbageOpen:
			s.garbage()
		}
	}
	return s.groupScore, s.garbageCount
}

func version2(input []byte) (int, int) {
	s := stream{input: input}
	return s.part1()
}

func version1(input []byte) (groupScore int, garbageCount int) {
	groupDepth := 0
	for dataPtr := 0; dataPtr < len(input); dataPtr++ {
		switch input[dataPtr] {
		case groupOpen:
			groupDepth++
		case groupClose:
			groupScore += groupDepth
			groupDepth--
		case garbageOpen:
			for dataPtr++; input[dataPtr] != garbageClose; dataPtr++ {
				if input[dataPtr] == ignore {
					dataPtr++
				} else {
					garbageCount++
				}
			}
		}
	}
	return groupScore, garbageCount
}

func main() {
	data, err := os.ReadFile("input.txt")
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error reading file: %v\n", err)
		os.Exit(1)
	}
	groupScore, garbageCount := version1(data)
	fmt.Printf("Version 1\nPart 1: %d\nPart 2: %d\n\n", groupScore, garbageCount)

	groupScore, garbageCount = version2(data)
	fmt.Printf("Version 2\nPart 1: %d\nPart 2: %d\n\n", groupScore, garbageCount)

}

// Expected Output
// Group Score: 12803
// Garbage Total: 6425
