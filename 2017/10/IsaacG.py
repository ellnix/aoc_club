import functools
import math
import more_itertools
from lib import aoc


INPUT_PARSER = aoc.parse_one_str


def tie_knots(lengths: list[int], size: int) -> list[int]:
    """Return list with knots tied at each length."""
    data = list(range(size))
    position = 0
    for skip, length in enumerate(lengths):
        # Reverse the length-span.
        data = list(reversed(data[:length])) + data[length:]
        # "Shift left" so the list starts at the position pointer.
        split = (length + skip) % size
        data = data[split:] + data[:split]
        position += length + skip

    # Shift the list based on the position so the position is actually offset and not at "0".
    split = size - (position % size)
    return data[split:] + data[:split]


def part1(parsed_input: str) -> int:
    """Tie knots and return the product of the first two values."""
    # Parse the input as a list of numbers.
    lengths = [int(i) for i in parsed_input.split(",")]
    size = 5 if testing else 256
    data = tie_knots(lengths, size)
    return math.prod(data[:2])


def part2(parsed_input: str) -> str:
    """Tie knots 64 times then densify and hexify."""
    # Parse the input as a list of ASCII bytes.
    lengths = [ord(i) for i in parsed_input] + [17, 31, 73, 47, 23]
    data = tie_knots(lengths * 64, 256)
    # Chunk and XOR in chunks of 16.
    dense = [
        functools.reduce(lambda a, b: a ^ b, chunk)
        for chunk in more_itertools.chunked(data, 16)
    ]
    # Format as two-char hex values.
    return "".join(f"{i:02x}" for i in dense)
