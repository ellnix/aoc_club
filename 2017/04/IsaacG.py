#!/bin/python
"""Day 4: High-Entropy Passphrases. Count valid passphrases."""

INPUT_PARSER = aoc.parse_multi_str_per_line


def part1(self, parsed_input: list[list[str]]) -> int:
    """Count passphrases where no word is repeated."""
    return sum(len(set(line)) == len(line) for line in parsed_input)


def part2(self, parsed_input: list[list[str]]) -> int:
    """Count passphrases where no word is an anagram of another."""
    return sum(
        all(sorted(a) != sorted(b) for a, b in itertools.combinations(line, 2))
        for line in parsed_input
    )
