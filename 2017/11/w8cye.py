# pylint: disable=missing-docstring

# https://www.redblobgames.com/grids/hexagons/

moves = {
    "n": (0, 1, -1),
    "ne": (1, 0, -1),
    "se": (1, -1, 0),
    "s": (0, -1, 1),
    "sw": (-1, 0, 1),
    "nw": (-1, 1, 0),
}


def day11(directions):
    x, y, z = 0, 0, 0
    distances = []
    for direction in directions:
        dx, dy, dz = moves[direction]
        x += dx
        y += dy
        z += dz
        distances.append((abs(x) + abs(y) + abs(z)) // 2)
    return distances[-1], max(distances)


if __name__ == "__main__":
    with open("input.txt", encoding="utf-8") as f:
        file_input = f.read()
    part1, part2 = day11(file_input.split(","))
    print(f"{part1=}")
    print(f"{part2=}")
