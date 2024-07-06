package day01

func SolveCaptcha(numbers string) int {
	totalLen := len(numbers)
	sum := 0

	for idx := range totalLen {
		next := (idx + (totalLen - 1)) % totalLen
		if numbers[next] != numbers[idx] {
			continue
		}
		sum += int(numbers[idx] - '0') // convert byte to its equivalent int.
	}
	return sum
}

func SolveCaptchaTwo(numbers string) int {
	totalLen := len(numbers)
	sum := 0
	step := totalLen / 2

	for idx := range totalLen {
		next := (idx + step) % totalLen

		if numbers[next] != numbers[idx] {
			continue
		}
		sum += int(numbers[idx] - '0')
	}
	return sum
}
