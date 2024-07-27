#!/bin/python

import functools
import more_itertools
from lib import aoc


def knot_hash(data: str) -> list[int]:
    """Implement a Knot Hash. Pulled from 2017/10 and shortened."""
    nums = [ord(i) for i in data] + [17, 31, 73, 47, 23]
    nums *= 64
    size = 256

    def tie_knots() -> list[int]:
        data = list(range(size))
        position = 0
        for skip, length in enumerate(nums):
            data = list(reversed(data[:length])) + data[length:]
            split = (length + skip) % size
            data = data[split:] + data[:split]
            position += length + skip

        split = size - (position % size)
        return data[split:] + data[:split]

    return [
        functools.reduce(lambda a, b: a ^ b, chunk)
        for chunk in more_itertools.chunked(tie_knots(), 16)
    ]


PARAMETERIZED_INPUTS = [True, False]
INPUT_PARSER = aoc.parse_one_str

def solver(parsed_input: str, param: bool) -> int:
    # Compute all the Knot Hashes as lines of binary values.
    lines = (
        "".join([f"{i:08b}" for i in knot_hash(f"{parsed_input}-{pos_y}")])
        for pos_y in range(128)
    )
    # Make a set of locations which are in use.
    used = {
        complex(pos_x, pos_y)
        for pos_y, line in enumerate(lines)
        for pos_x, val in enumerate(line)
        if val == "1"
    }
    # Part one: return the number of locations which are in use.
    if param:
        return len(used)

    # Part two: count in use islands.
    count = 0
    while used:
        # Select any in use location and expand outwards until we consume the entire island.
        todo = {used.pop()}
        count += 1
        while todo:
            todo = {
                neighbor
                for position in todo
                for neighbor in aoc.neighbors(position)
                if neighbor in used
            }
            used -= todo
    return count
