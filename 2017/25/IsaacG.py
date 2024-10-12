#!/bin/python

import re
from lib import aoc
InputType = tuple[str, int, dict[str, list[tuple[bool, int, str]]]]


def solver(puzzle_input: InputType, part_one: bool) -> int:
    cursor = 0
    state, steps, rules = puzzle_input
    tape = set()
    for _ in range(steps):
        write, move, state = rules[state][cursor in tape]
        if write:
            tape.add(cursor)
        else:
            tape.discard(cursor)
        cursor += move
    return len(tape)

def input_parser(puzzle_input: str) -> InputType:
    """Parse the input data."""
    preamble, *rules = puzzle_input.split("\n\n")
    patt = r"Begin in state (.)\.\nPerform a diagnostic checksum after (\d+) steps."
    m = re.match(patt, preamble)
    assert m
    initial_state = m.group(1)
    steps = int(m.group(2))

    patt = re.compile(r"""In state (.):
If the current value is 0:
- Write the value ([01])\.
- Move one slot to the (left|right)\.
- Continue with state (.)\.
If the current value is 1:
- Write the value ([01])\.
- Move one slot to the (left|right)\.
- Continue with state (.)\.""")

    res = {}
    for rule in rules:
        m = patt.match(rule)
        assert m
        start, w0, m0, n0, w1, m1, n1 = m.groups()
        res[start] = [
                (w0 == "1", 1 if m0 == "right" else -1, n0),
                (w1 == "1", 1 if m1 == "right" else -1, n1),
        ]
    return initial_state, steps, res
