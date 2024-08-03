# pylint: disable=W0621, C0115, C0116
from functools import reduce
from itertools import batched
from operator import xor
from pathlib import Path
from time import perf_counter_ns


def part1(data) -> int:
    return solve(data, True) 

def part2(data) -> int:
    return solve(data, False)

def solve(data, part2=False) -> int:
    disk: set[tuple[int,int]] = set()
    for i in range(128):
        hex_string = computeDenseHash(f"{data}-{i}")
        binary_string = bin(int(hex_string, 16))[2:].zfill(128)
        disk.update((i, j) for j, bit in enumerate(binary_string) if bit == "1")

    if part2:
        return len(disk)

    total = 0
    while disk:
        total += 1
        to_visit = {disk.pop()}
        while to_visit:
            visit = to_visit.pop()
            for neighbor in orthogonal_neighbors(*visit):
                if neighbor in disk:
                    to_visit.add(neighbor)
                    disk.remove(neighbor)
    return total


def orthogonal_neighbors(x, y: int) -> list[tuple[int, int]]:
    return [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]


def computeDenseHash(data):
    lengths = [ord(n) for n in data] + [17, 31, 73, 47, 23]
    sparse_hash = knot_hash(lengths * 64)
    dense_hash = [reduce(xor, block) for block in batched(sparse_hash, 16)]
    return "".join(f"{hex_part:02x}" for hex_part in dense_hash)


def knot_hash(lengths):
    sparse_hash = [i for i in range(256)]
    current_position = 0
    for skip_size, length in enumerate(lengths):
        sparse_hash = sparse_hash[:length][::-1] + sparse_hash[length:]
        shift = (length + skip_size) % len(sparse_hash)
        sparse_hash = sparse_hash[shift:] + sparse_hash[:shift]
        current_position = (current_position + shift) % len(sparse_hash)
        skip_size += 1

    realignment_offset = len(sparse_hash) - current_position
    sparse_hash = sparse_hash[realignment_offset:] + sparse_hash[:realignment_offset]
    return sparse_hash


if __name__ == "__main__":
    with Path("input.txt").open(encoding="utf-8") as f:
        file_input = f.read()
    data = file_input.strip()

    for m in [part1, part2]:
        start = perf_counter_ns()
        result = m(data)
        end = perf_counter_ns()
        print(f"{m.__name__:12} {result:12,} {end-start:15,} ns")
