#!/bin/python
"""Advent of Code, Day 3: Spiral Memory."""

ROTATE_LEFT = 1j


class Day03(aoc.Challenge):
    """Day 3: Spiral Memory."""

    PARAMETERIZED_INPUTS = [False, True]
    INPUT_PARSER = aoc.parse_one_int

    def solver(self, parsed_input: int, param: bool) -> int:
        """Fill out a spiral matrix until we get to the input number."""
        location = complex(0, 0)
        direction = complex(0, -1)
        number = 1

        matrix = {location: number}
        for number in range(2, parsed_input + 1):
            if location + direction * ROTATE_LEFT not in matrix:
                direction *= ROTATE_LEFT
            location += direction
            if param:
                value = sum(
                    matrix.get(location + offset, 0) for offset in aoc.EIGHT_DIRECTIONS)
            else:
                value = number
            matrix[location] = value
            if param and value > parsed_input:
                return value

        return int(abs(location.real) + abs(location.imag))
