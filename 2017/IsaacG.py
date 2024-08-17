INPUT_PARSER = aoc.parse_one_int


def part1(parsed_input: int) -> int:
    """Step and insert 2017 times then return the next value."""
    step = parsed_input
    data = [0]
    pos = 0
    for i in range(1, 2017 + 1):
        pos = (pos + step) % i + 1
        data.insert(pos % (i + 1), i)

    return data[(pos + 1) % len(data)]


def part2(parsed_input: int) -> int:
    """Return the value after 0 after 50M step-inserts."""
    step = parsed_input
    pos = 0
    last_insert = 0
    for i in range(1, 50000000 + 1):
        pos = ((pos + step) % i) + 1
        if pos == 1:
            last_insert = i

    return last_insert
