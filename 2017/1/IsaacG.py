#!/bin/python
"""Advent of Code, Day 1: Inverse Captcha."""

class Day01(aoc.Challenge):
    """Day 1: Inverse Captcha."""

    INPUT_PARSER = aoc.parse_one_str
    PARAMETERIZED_INPUTS = (
        # Part one: compare digits to subsequent digit.
        lambda x: 1,
        # Part two: compare digits to digits halfway around the list.
        lambda x: len(x) // 2,
    )

    def solver(self, parsed_input: str, offsetter: Callable[[str], int]) -> int:
        """Sum all digits which match the corresponding digit."""
        offset = offsetter(parsed_input)
        shifted_list = parsed_input[offset:] + parsed_input[:offset]
        return sum(
            int(digit)
            for digit, corresponding in zip(parsed_input, shifted_list)
            if digit == corresponding
        )
