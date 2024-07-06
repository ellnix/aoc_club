package day07

import (
	"errors"
	"fmt"
	"strconv"
	"strings"
)

// link an parent to its children nodes
// save the weight of all programs in a hashmap
// use BFS to search through the tree and find the total weight

type wrongWeightError struct {
	badProgram string
	expected   int
}

func (w *wrongWeightError) Error() string {
	return fmt.Sprintf("for program %s expected weight %d", w.badProgram, w.expected)
}

type tree struct {
	start  string
	link   map[string][]string
	weight map[string]int
}

func newTree(input []string) (*tree, error) {
	fParent := make(map[string]string)
	link := make(map[string][]string)
	weight := make(map[string]int)
	var start string
	for _, inst := range input {
		split := strings.Split(inst, " -> ")
		temp := strings.Fields(split[0])
		mTower := temp[0]
		w, err := strconv.Atoi(strings.Trim(temp[1], "()"))
		if err != nil {
			return nil, err
		}
		weight[mTower] = w
		if len(split) == 1 {
			continue
		}
		subTowers := strings.Split(split[1], ", ")
		link[mTower] = subTowers
		for _, t := range subTowers {
			fParent[t] = mTower
		}
		start = mTower
	}
	var res string
	for start != "" {
		res = start
		start = fParent[start]
	}
	return &tree{start: res, link: link, weight: weight}, nil
}

func (c *tree) bfsTree(prog string) (int, error) {
	if _, ok := c.link[prog]; !ok {
		return c.weight[prog], nil
	}
	var w []int
	for _, subProg := range c.link[prog] {
		num, err := c.bfsTree(subProg)
		if err != nil {
			return 0, err
		}
		w = append(w, num)
	}
	val, idx := findOdd(w)
	if val != 0 {
		badProg := c.link[prog][idx]
		nxt := (idx + 1) % len(w)
		diff := w[nxt] - val
		expW := c.weight[badProg] + diff
		return 0, &wrongWeightError{badProgram: badProg, expected: expW}
	}
	return c.weight[prog] + (w[0] * len(w)), nil
}

func findOdd(w []int) (int, int) {
	for i := 1; i < len(w)-1; i++ {
		prev, cur, next := w[i-1], w[i], w[i+1]
		switch {
		case prev == cur && cur != next:
			return next, (i + 1)
		case prev == next && prev != cur:
			return cur, i
		case cur == next && cur != prev:
			return prev, (i - 1)
		}
	}
	return 0, 0
}

func SolveOne(input []string) (string, error) {
	c, err := newTree(input)
	if err != nil {
		return "", err
	}
	return c.start, nil
}

func SolveTwo(input []string) (string, error) {
	c, err := newTree(input)
	if err != nil {
		return "", err
	}
	_, err = c.bfsTree(c.start)
	var ww *wrongWeightError
	if !errors.As(err, &ww) {
		return "", err
	}
	return err.Error(), nil
}
