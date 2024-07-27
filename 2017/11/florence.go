package day11

type coord struct {
	x, y int
}

func SolveOne(input []string) (int, error) {
	var pos int
	solve(input, func(i int) { pos = i })
	return pos, nil
}

func SolveTwo(input []string) (int, error) {
	var pos int
	solve(input, func(i int) { pos = max(pos, i) })
	return pos, nil
}

func solve(input []string, yield func(int)) {
	pos := coord{0, 0}
	for _, dir := range input {
		offSet := dirToCoord[dir]
		pos.x += offSet.x
		pos.y += offSet.y
		yield(max(pos.x, pos.y, -pos.x, -pos.y))
	}
}

var dirToCoord = map[string]coord{
	"n":  {1, 1},
	"ne": {1, 0},
	"nw": {0, 1},
	"s":  {-1, -1},
	"se": {0, -1},
	"sw": {-1, 0},
}

// func dirToCoord(dir string) coord {
// 	switch dir {
// 	case "n":
// 		return coord{1, 1}
// 	case "ne":
// 		return coord{1, 0}
// 	case "nw":
// 		return coord{0, 1}
// 	case "s":
// 		return coord{-1, -1}
// 	case "se":
// 		return coord{0, -1}
// 	case "sw":
// 		return coord{-1, 0}
// 	}
// 	return coord{0, 0}
// }
