# pylint: disable=missing-docstring

from pathlib import Path
from time import perf_counter_ns


class Dance:
    def __init__(self, data: list[str]) -> None:
        self.progs = list("abcdefghijklmnop")
        self.moves = self.parse(data)

    def parse(self, data: list[str]) -> list:
        moves = []
        for move in data:
            match move[0], move[1:].split("/"):
                case "s", [step]:
                    moves.append((self.spin, int(step))) # type: ignore[arg-type]
                case "x", (a, b):
                    moves.append((self.exchange, int(a), int(b))) # type: ignore[arg-type]
                case "p", (a, b):
                    moves.append((self.partner, a, b)) # type: ignore[arg-type]
        return moves

    def spin(self, x: int) -> None:
        self.progs = self.progs[-x:] + self.progs[:-x]

    def exchange(self, a: int, b: int) -> None:
        self.progs[a], self.progs[b] = self.progs[b], self.progs[a]

    def partner(self, a: str, b: str) -> None:
        a_idx, b_idx = self.progs.index(a), self.progs.index(b)
        self.progs[a_idx], self.progs[b_idx] = self.progs[b_idx], self.progs[a_idx]

    def part1(self) -> str:
        for command, *params in self.moves:
            command(*params)
        return "".join(self.progs)

    def part2(self) -> str:
        seen: list[str] = []
        for i in range(1_000_000_000):
            dance = self.part1()
            if dance in seen:
                return seen[999_999_999 % i]
            seen.append(dance)
        return seen[-1]

    def part2b(self) -> str:
        seen: list[str] = []
        for i in range(1_000_000_000):
            dance = self.part1()
            if dance == "abcdefghijklmnop":
                break
            seen.append(dance)
        return seen[999_999_999 % (i + 1)]


if __name__ == "__main__":
    with Path("input.txt").open(encoding="utf-8") as f:
        file_input = f.read()

    data = []
    for line in file_input.splitlines():
        data.extend(line.split(","))

    for m in ["part1","part2", "part2b"]:
        d = Dance(data)
        start = perf_counter_ns()
        result = getattr(d, m)()
        end = perf_counter_ns()
        print(f"{m:12} {result:12} {end-start:15,} ns")
