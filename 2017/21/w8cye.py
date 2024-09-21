# pylint: disable=C0114,C0115,C0116
from pathlib import Path
from itertools import batched


def string_to_grid(s):
    return tuple(tuple(line) for line in s.strip().split("/"))


def rotate(grid):
    return tuple(zip(*grid[::-1]))


def flip(grid):
    return tuple(row[::-1] for row in grid)


def chunk(grid, chunk_size):
    chunked_rows = (
        tuple(tuple(row_chunk) for row_chunk in batched(row, chunk_size))
        for row in grid
    )
    return (tuple(zip(*col_chunk)) for col_chunk in batched(chunked_rows, chunk_size))


def get_transformations(grid):
    for _ in range(4):
        yield grid
        yield flip(grid)
        grid = rotate(grid)


def parse_input(file_input):
    book = {}
    for line in file_input.strip().splitlines():
        key, value = line.strip().split(" => ")
        key_grid = string_to_grid(key)
        value_grid = string_to_grid(value)
        for trans in get_transformations(key_grid):
            book[trans] = value_grid
    return book


def solve(iterations, file_input):
    book = parse_input(file_input)

    def enhance(grid: tuple) -> tuple:
        new_grid = []
        chunk_size = 2 + len(grid) % 2
        for patterns in chunk(grid, chunk_size):
            new_patterns = (book[pattern] for pattern in patterns)
            # unchunk
            for row in tuple(sum(rows, ()) for rows in zip(*new_patterns)):
                new_grid.append(row)
        return tuple(new_grid)

    grid = string_to_grid(".#./..#/###")
    for _ in range(iterations):
        grid = enhance(grid)
    return sum(row.count("#") for row in grid)


if __name__ == "__main__":
    with Path("input.txt").open(encoding="utf-8") as f:
        file_input = f.read()

    print(f"Part 1: {solve(5,file_input)}")
    print(f"Part 2: {solve(18,file_input)}")
