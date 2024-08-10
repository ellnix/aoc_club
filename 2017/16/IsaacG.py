#!/bin/python

import collections
import string

InputType = list[tuple[int, str | int, str | int]]
PARAMETERIZED_INPUTS = [False, True]

SPIN = 0
EXCHANGE = 1
PARTNER = 2

def solver(self, parsed_input: InputType, param: bool) -> str:
    size = 16
    dances = 1000000000 if param else 1
    cmds = parsed_input

    dance_line = collections.deque(string.ascii_lowercase[:size])

    def dance():
        for op, first, second in cmds:
            if op == SPIN:
                dance_line.rotate(first)
            elif op == EXCHANGE:
                dance_line[first], dance_line[second] = dance_line[second], dance_line[first]
            else:
                a, b = dance_line.index(first), dance_line.index(second)
                dance_line[a], dance_line[b] = dance_line[b], dance_line[a]

    seen = {}
    for count in range(dances):
        dance()
        t = tuple(dance_line)
        if t in seen:
            remaining = (dances - 1 - count) % (count - seen[t])
            for _ in range(remaining):
                dance()
            break
        seen[t] = count

    return "".join(dance_line)


def input_parser(self, puzzle_input: str) -> InputType:
    cmds = []
    for word in puzzle_input.split(","):
        match word[0], word[1:].split("/"):
            case "s", [step]:
                cmds.append((SPIN, int(step), 0))
            case "x", (first, second):
                cmds.append((EXCHANGE, int(first), int(second)))
            case "p", (first, second):
                cmds.append((PARTNER, first, second))
    return cmds
