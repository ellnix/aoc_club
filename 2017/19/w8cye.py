from pathlib import Path


def part1and2(
    data: dict[tuple[int, int], str], start: tuple[int, int]
) -> tuple[str, int]:
    dir_row, dir_col = 1, 0
    pos_row, pos_col = start
    result = []
    steps = 0
    while cell := data.get((pos_row, pos_col)):
        steps += 1
        if cell == "+":
            if dir_row == 0:
                dir_row = 1 if data.get((pos_row + 1, pos_col)) else -1
                dir_col = 0
            else:
                dir_col = 1 if data.get((pos_row, pos_col + 1)) else -1
                dir_row = 0
        elif cell.isalpha():
            result.append(cell)
        pos_row += dir_row
        pos_col += dir_col
    return "".join(result), steps


if __name__ == "__main__":
    with Path("input.txt").open(encoding="utf-8") as f:
        file_input = f.read()

    start = (0, file_input.find("|"))
    data = {}

    for row, line in enumerate(file_input.splitlines()):
        for col, char in enumerate(line):
            if char != " ":
                data[(row, col)] = char

    part1, part2 = part1and2(data, start)
    print(f"Part 1: {part1}")
    print(f"Part 2: {part2}")
