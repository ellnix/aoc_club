#!/bin/python

import itertools

PARAMETERIZED_INPUTS = [False, True]
INPUT_PARSER = aoc.parse_multi_int_per_line


def solver(self, parsed_input: list[list[int]], param: bool) -> int:
    banks = parsed_input[0]
    seen: dict[int, int] = {}
    count = len(banks)

    for step in itertools.count():
        # Use a hash to be more efficient than storing the whole tuple.
        hash_ = hash(tuple(banks))
        # Check for loops.
        if hash_ in seen:
            # Return the current step (p1) vs the loop length (p2).
            return step - seen[hash_] if param else step
        seen[hash_] = step

        # Sort by block count then (reverse) bank ID.
        num, idx = max((num, -idx) for idx, num in enumerate(banks))
        idx = -idx

        # Reallocate blocks.
        banks[idx] = 0
        for bank in range(idx + 1, idx + 1 + num):
            banks[bank % count] += 1
