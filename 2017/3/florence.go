package day03

import (
	"math"
)

/*
37 36  35  34  33  32  31
38 17  16  15  14  13  30
39 18   5   4   3  12  29
40 19   6   1   2  11  28
41 20   7   8   9  10  27
42 21  22  23  24  25  26
43 44  45  46  47  48  49
*/

// lets find x and y,
// each corner is ((i*2-1)+2)^2
// to get x position in underroot(x)/2  and then round it, works for x coordinate.
// try 34, 27, 49, 21

func SolveOne(input int) int {
	x := roundToInt(math.Sqrt(float64(input)) / 2)
	if x == 0 {
		return x
	}
	y := ((input - (calLast(x-1) + 1)) % (x * 2)) - (x - 1)
	if y < 0 {
		return x + (y * -1)
	}
	return x + y
}

func calLast(x int) int {
	res := ((x * 2) - 1) + 2
	return res * res
}

func roundToInt(x float64) int {
	if i := float64(int(x)+1) - x; i < 0.5 {
		return int(x) + 1
	}
	return int(x)
}

/*

147  142  133  122   59
304    5    4    2   57
330   10    1    1   54
351   11   23   25   26
362  747  806  880  931


37 36  35  34  33  32  31
38 17  16  15  14  13  30
39 18   5   4   3  12  29
40 19   6   1   2  11  28
41 20   7   8   9  10  27
42 21  22  23  24  25  26
43 44  45  46  47  48  49

*/
// maybe we could use recursion, with memoization.
// there is a pattern so look for that.
// lets use isaac's method of using hashmaps as the matrix

type direction int

const (
	right direction = iota
	left
	top
	bottom
	topRight
	topLeft
	bottomRight
	bottomLeft
)

type coord struct {
	x, y int
}

var dir = [...]coord{ //nolint:gochecknoglobals
	top:         {0, 1},
	right:       {1, 0},
	bottom:      {0, -1},
	left:        {-1, 0},
	topLeft:     {-1, 1},
	topRight:    {1, 1},
	bottomLeft:  {-1, -1},
	bottomRight: {1, -1},
}

func SolveTwo(input int) int {
	mtx := map[coord]int{ // hardcode first 2.
		{x: 0, y: 0}: 1,
		{x: 1, y: 0}: 1,
	}
	step := 0
	x, y := 1, 0
	for {
		step += 2
		pattern := []struct {
			dir direction
			rep int
		}{
			{top, (step - 1)},
			{left, step},
			{bottom, step},
			{right, (step + 1)},
		}
		for _, pat := range pattern {
			for range pat.rep {
				x, y = (dir[pat.dir].x + x), (dir[pat.dir].y + y)
				val := lookAround(mtx, x, y)
				if val > input {
					return val
				}
				mtx[coord{x: x, y: y}] = val
			}
		}
	}
}

func lookAround(mtx map[coord]int, x, y int) int {
	total := 0
	for _, off := range dir {
		num, ok := mtx[coord{x: (x + off.x), y: (y + off.y)}]
		if !ok {
			continue
		}
		total += num
	}
	return total
}
