package day09

// at each normal encounter of {, add one to elevation var,
// and at each encounter of }, add the current elevation var
// to the total (our result).
// use a bool flag to indicate if we are inside the garbage of not
// if we are then just continue untile garbage if closed.
// no runes in the input, so thats good.

func SolveOne(input string) int {
	score, _ := solve(input)
	return score
}

func SolveTwo(input string) int {
	_, garbChar := solve(input)
	return garbChar
}

func solve(input string) (score int, garbChar int) {
	var elev int
	var insideGarbage bool
	for i := 0; i < len(input); i++ {
		if insideGarbage {
			switch input[i] {
			case '!':
				i++
			case '>':
				insideGarbage = false
			default:
				garbChar++
			}
			continue
		}
		switch input[i] {
		case '{':
			elev++
		case '}':
			score += elev
			elev--
		case '<':
			insideGarbage = true
		}
	}
	return
}
