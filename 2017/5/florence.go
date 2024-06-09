package day05

func SolveOne(input []int) int {
	return solve(input, func(i int) int { return i + 1 })
}

func SolveTwo(input []int) int {
	return solve(input, func(i int) int {
		if i >= 3 {
			return i - 1
		}
		return i + 1
	})
}

func solve(input []int, procFunc func(int) int) int {
	step := 0
	for i := 0; i < len(input); {
		jump := input[i]
		input[i] = procFunc(input[i])
		i += jump
		step++
	}
	return step
}
