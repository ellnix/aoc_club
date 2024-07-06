#!/bin/python

import collections
import operator

OPERATORS = {
    ">": operator.gt,
    ">=": operator.ge,
    "<": operator.lt,
    "<=": operator.le,
    "==": operator.eq,
    "!=": operator.ne,
    # ---
    "dec": operator.sub,
    "inc": operator.add,
}


INPUT_PARSER = aoc.parse_multi_str_per_line
PARAMETERIZED_INPUTS = [1, 2]

def solver(parsed_input: list[list[str]], param: int) -> int:
    largest = 0
    registers: dict[str, int] = collections.defaultdict(int)
    for target_reg, action, action_amount, _, test_reg, test_op, test_amount in parsed_input:
        if OPERATORS[test_op](registers[test_reg], int(test_amount)):              # Perform the check
            result = OPERATORS[action](registers[target_reg], int(action_amount))  # Compute the new value
            largest = max(largest, result)                                         # Track the max
            registers[target_reg] = result                                         # Update the register

    if param == 1:
        return max(registers.values())
    return largest
