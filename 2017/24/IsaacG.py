def solver(self, puzzle_input: frozenset[tuple[int, int]], part_one: bool) -> int:

    def strongest(start: int, options: frozenset[tuple[int, int]]) -> tuple[int, int, int]:
        strength = 0
        longest_length = 0
        longest_strength = 0

        for option in options:
            if start not in option:
                continue
            cur = option[0] + option[1]
            next_start = cur - start
            n_str, n_len, n_long_str = strongest(next_start, options - {option})
            strength = max(strength, n_str + cur)
            n_long_str += cur
            if n_len > longest_length:
                longest_length = n_len
                longest_strength = n_long_str
            elif n_len == longest_length and n_long_str > longest_strength:
                longest_strength = n_long_str
        return strength, longest_length + 1, longest_strength

    return strongest(0, puzzle_input)[0 if part_one else 2]

def input_parser(self, puzzle_input: str) -> frozenset[tuple[int, int]]:
    lines = puzzle_input.splitlines()
    pairs = {tuple(sorted(int(i) for i in line.split("/"))) for line in lines}
    assert len(pairs) == len(lines)
    return frozenset(pairs)  # type: ignore
