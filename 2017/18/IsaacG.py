#!/bin/python
"""Advent of Code, Day 18: Duet."""

def program(
    code: list[list[str | int]],
    program_id: int,
    part_one: bool,
) -> collections.abc.Generator[list[int], list[int], int]:
    """Run a program with IO queues.

    On snd, collect values in a list to return.
    On rcv, if we have prior values, use those.
    Otherwise, yield the collected values and receive multiple inputs.
    If we yield and get nothing back, we are stuck. Return send count.

    Part 1 returns the last snd value on rcv non-zero.
    Part 2 yields on a rcv and returns the total send count.
    """
    registers = collections.defaultdict(int)
    registers["p"] = program_id
    ptr = 0
    sent = 0
    outputs: list[int] = []  # Used to collect multiple snd values for one yield.
    inputs = yield outputs  # Initial yield is to set things up without needing to handle outputs.
    q_in = collections.deque(inputs)

    def val(register: int | str) -> int:
        """Helper. Return a value, either an immediate value or register lookup."""
        if isinstance(register, int):
            return register
        return registers[register]

    while True:
        instructions = code[ptr]
        ptr += 1
        match instructions:
            case ["set", str(X), Y]:
                registers[X] = val(Y)
            case ["add", str(X), Y]:
                registers[X] += val(Y)
            case ["mul", str(X), Y]:
                registers[X] *= val(Y)
            case ["mod", str(X), Y]:
                registers[X] %= val(Y)
            case ["jgz", X, Y] if val(X) > 0:
                ptr += val(Y) - 1
            case ["snd", X]:
                outputs.append(val(X))
                sent += 1
            case ["rcv", X] if part_one and val(X):
                # Part one. On the first non-zero rcv, return the last send value.
                return outputs.pop()
            case ["rcv", str(X)] if not part_one:
                # Yield if we are out of values.
                # If we have recv values, read one.
                # If we yielded and still do not have values, return the sent count.
                if not q_in:
                    inputs = yield outputs
                    outputs = []
                    q_in.extend(inputs)
                    if not inputs:
                        return sent
                registers[X] = q_in.popleft()


INPUT_PARSER = aoc.parse_multi_mixed_per_line
PARAMETERIZED_INPUTS = [True, False]

def solver(program_instructions: list[list[str | int]], part_one: bool) -> int:
    """Run two programs and get IO details."""
    # Create two programs.
    programs = {
        i: program(program_instructions, i, part_one)
        for i in range(2)
    }
    # Run the programs to the first yield, which always returns [].
    # This is part of initializing the program states.
    next(programs[0])
    vals = next(programs[1])
    # Alternate between programs.
    for i in itertools.cycle(programs):
        # A try-except is needed to run until a `return` occurs.
        try:
            # Send in values from other program and read out values for the next program.
            # Chain values from one program to the other.
            vals = programs[i].send(vals)
        except StopIteration as e:
            # On a StopIteration (triggered by a return), handle the return value.
            if part_one or i == 1:
                return e.value
    raise RuntimeError("We should never get to this point.")
