#!/bin/python

class Day05(aoc.Challenge):
    PARAMETERIZED_INPUTS = [False, True]
    INPUT_PARSER = aoc.parse_one_int_per_line

    def solver(self, parsed_input: list[int], param: bool) -> int:
        mem = parsed_input.copy()
        size = len(mem)
        ptr = 0

        step = 0
        while 0 <= ptr < size:
            offset = mem[ptr]
            mem[ptr] += -1 if param and offset >= 3 else 1
            ptr += offset
            step += 1
        return step
