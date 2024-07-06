#!/bin/python
"""Advent of Code, Day 2: Corruption Checksum."""


class Day02(aoc.Challenge):
    """Day 2: Corruption Checksum. Compute checksums of lines."""

    INPUT_PARSER = aoc.parse_multi_int_per_line

    def part1(self, parsed_input: list[list[int]]) -> int:
        """Return the sum difference between the largest and smallest value on each line."""
        return sum(
            max(line) - min(line)
            for line in parsed_input
        )

    def part2(self, parsed_input: list[list[int]]) -> int:
        """Return the sum quotient between two evenly divisible numbers on each line."""
        return sum(
            next(
                a // b
                for a, b in itertools.combinations(sorted(line, reverse=True), 2)
                if a % b == 0
            )
            for line in parsed_input
        )
