package day02

func SolveOne(input [][]int) int {
	total := 0
	for idx := range input {
		small := input[idx][0]
		large := input[idx][0]
		for _, num := range input[idx][1:] {
			small = min(small, num)
			large = max(large, num)
		}
		total += (large - small)
	}
	return total
}

func SolveTwo(input [][]int) int {
	total := 0
	for _, row := range input {
	rowLoop:
		for idx := range len(row) - 1 {
			for i := idx + 1; i < len(row); i++ {
				if row[idx]%row[i] == 0 {
					total += (row[idx] / row[i])
					break rowLoop
				}

				if row[i]%row[idx] == 0 {
					total += (row[i] / row[idx])
					break rowLoop
				}
			}
		}
	}
	return total
}
