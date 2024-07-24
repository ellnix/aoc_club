# pylint: disable=missing-docstring
import re
from time import perf_counter_ns
from math import lcm
from pathlib import Path


class Interval:  # pylint: disable=too-few-public-methods
    def __init__(self, height: int, parsed_input: list[tuple[int, int]]) -> None:
        self.height = height
        self.interval = (height - 1) * 2
        self.depths = {depth for depth, h in parsed_input if h == height}
        self.reducible = (len(self.depths) == self.height - 2) or (
            self.interval == 2 and len(self.depths) == 1
        )

    def blocked(self, delay: int) -> bool:
        return any((depth + delay) % self.interval == 0 for depth in self.depths)


class Firewall:
    parsed_input: list[tuple[int, int]]

    def __init__(self, parsed_input: list[tuple[int, int]]) -> None:
        self.parsed_input = parsed_input

    def part1(self) -> int:
        return sum(
            depth * height for depth, height in self.parsed_input if depth % (2 * (height - 1)) == 0
        )

    def part2_slow(self) -> int:
        delay = 0
        while not all((delay + depth) % (2 * (height - 1)) for depth, height in self.parsed_input):
            delay += 1
        return delay

    def part2_faster(self) -> int:
        reducible = []
        remainder = []

        for height in set(height for _, height in self.parsed_input):
            interval = Interval(height, self.parsed_input)
            if interval.reducible:
                reducible.append(interval)
            else:
                remainder.append(interval)

        step = lcm(*[interval.interval for interval in reducible])
        delay = self.find_delay(reducible)

        return self.find_delay(remainder, delay, step)

    def find_delay(self, intervals: list[Interval], delay: int = 0, step: int = 1) -> int:
        while any(interval.blocked(delay) for interval in intervals):
            delay += step
        return delay


if __name__ == "__main__":
    with Path("input.txt").open(encoding="utf-8") as f:
        file_input = f.read()

    data = []

    for line in file_input.splitlines():
        key, value = (int(n) for n in re.split(r"\D+", line))
        data.append((int(key), int(value)))

    firewall = Firewall(data)

    for m in ["part1", "part2_slow", "part2_faster"]:
        start = perf_counter_ns()
        result = getattr(firewall, m)()
        end = perf_counter_ns()
        print(f"{m:12} {result:12,} {end-start:15,} ns")

# part1               x,xxx           6,333 ns
# part2_slow      x,xxx,xxx   2,083,055,000 ns
# part2_faster    x,xxx,xxx       8,150,209 ns
