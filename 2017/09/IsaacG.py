INPUT_PARSER = aoc.parse_one_str
PARAMETERIZED_INPUTS = [0, 1]

def solver(self, datastream: str, param: int) -> int:
    # Stream of tokens to work through.
    tokens = iter(datastream)
    # Total score, current bracket score and number of garbage bytes.
    total_score = bracket_depth = garbage_count = 0
    garbage_group = False

    for char in tokens:                  # Pop the next token.
        if char == "!":
            next(tokens)                 # Ignore the next token.
        elif garbage_group:
            if char == ">":              # End of garbage.
                garbage_group = False
            else:
                garbage_count += 1       # Count garbage bytes.
        elif char == "<":
            garbage_group = True         # Start garbage stream.
        elif char == "{":
            bracket_depth += 1           # Enter nested bracket.
        elif char == "}":
            total_score += bracket_depth # Update score.
            bracket_depth -= 1           # Exit nested bracket.

    # Return total score for part one, number of garbage bytes for part two.
    return (total_score, garbage_count)[param]
