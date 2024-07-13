# pylint: disable=missing-docstring

import re


def part1(data):
    graph = data.copy()
    to_visit = {0}
    while to_visit:
        to_visit.update(graph.pop(to_visit.pop()) & graph.keys())
    return len(data) - len(graph)


def part2(data):
    graph = data.copy()
    groups = 0
    while graph:
        groups += 1
        to_visit = set(graph.popitem()[1]) & graph.keys()
        while to_visit:
            to_visit.update(graph.pop(to_visit.pop()) & graph.keys())
    return groups


if __name__ == "__main__":
    with open("input.txt", encoding="utf-8") as f:
        file_input = f.read()

    d = {}
    for line in file_input.splitlines():
        key, *values = (int(n) for n in re.split(r"\D+", line))
        d[key] = set(values)

    print(f"{part1(d)=}")
    print(f"{part2(d)=}")


# part1(d)=152
# part2(d)=186
