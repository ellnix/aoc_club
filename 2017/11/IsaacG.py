# See https://www.redblobgames.com/grids/hexagons/#distances for offsets and axial_distance().
PARAMETERIZED_INPUTS = [True, False]
INPUT_PARSER = aoc.parse_one_str

def axial_distance(current: complex) -> int:
    q, r = int(current.real), int(current.imag)
    return (abs(q) + abs(q + r) + abs(r)) // 2


def solver(self, parsed_input: str, part_one: bool) -> int:
    offsets = {
      'n':  +0 +1j,
      's':  +0 -1j,
      'ne': +1 +0j,
      'nw': -1 +1j,
      'se': +1 -1j,
      'sw': -1 +0j,
    }
    current = complex(0, 0)
    farthest = 0

    for direction in parsed_input.split(","):
        current += offsets[direction]
        farthest = max(farthest, axial_distance())

    return axial_distance() if part_one else farthest
