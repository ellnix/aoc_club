# pylint: disable=missing-docstring
from pathlib import Path
from typing import Self
from dataclasses import dataclass, field
from collections import defaultdict
import re


@dataclass(order=True)
class Vector:
    distance: int = field(init=False)
    x: int
    y: int
    z: int

    def __post_init__(self) -> None:
        self.update_distance()

    def __iadd__(self, other: Self) -> Self:
        self.x += other.x
        self.y += other.y
        self.z += other.z
        self.update_distance()
        return self

    def distance_to(self, other: Self) -> int:
        return abs(self.x - other.x) + abs(self.y - other.y) + abs(self.z - other.z)

    def update_distance(self) -> None:
        self.distance = abs(self.x) + abs(self.y) + abs(self.z)

    def __repr__(self) -> str:
        return f"({self.x},{self.y},{self.z})"

    def get_coords(self) -> tuple[int, int, int]:
        return (self.x, self.y, self.z)


@dataclass(order=True)
class Particle:
    acc: Vector = field(init=False)
    vel: Vector = field(init=False)
    pos: Vector = field(init=False)
    input_string: str
    index: int

    def __hash__(self) -> int:
        return self.index

    def __post_init__(self):
        matches = re.findall(r"<([^>]+)>", self.input_string)
        vals = [int(val) for match in matches for val in match.split(",")]
        self.pos = Vector(*vals[:3])
        self.vel = Vector(*vals[3:6])
        self.acc = Vector(*vals[6:])
        self.previous_pos = Vector(*vals[:3])

    def advance(self):
        self.vel += self.acc
        self.previous_pos = Vector(self.pos.x, self.pos.y, self.pos.z)
        self.pos += self.vel

    def __repr__(self) -> str:
        return f"Particle({self.index=},{self.pos=},{self.vel=},{self.acc=})"


def part1(input_data: str) -> int:
    data = [Particle(line, i) for i, line in enumerate(input_data.splitlines())]
    return sorted(data)[0].index


def part2(input_data: str) -> int:
    data = [Particle(line, i) for i, line in enumerate(input_data.splitlines())]
    for i in range(50):
        seen: dict[tuple[int, int, int], int] = defaultdict(int)
        for particle in data:
            particle.advance()
            seen[particle.pos.get_coords()] += 1
        data = [particle for particle in data if seen[particle.pos.get_coords()] == 1]

    return len(data)


if __name__ == "__main__":
    with Path("input.txt").open(encoding="utf-8") as f:
        file_input = f.read()

    print(f"Part 1: {part1(file_input)}")
    print(f"Part 2: {part2(file_input)}")
