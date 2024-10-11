from aocd import data
from aocd.models import Puzzle
from collections import defaultdict


def parse_data(data):
    graph = defaultdict(lambda: defaultdict(int))
    for line in data.splitlines():
        port_1, port_2 = map(int, line.split("/"))
        graph[port_1][port_2] += 1
        graph[port_2][port_1] += 1

    return graph


def part1(graph):
    def dfs(graph, vertex):
        max_strength = vertex

        for neighbor in graph[vertex].keys():
            if graph[vertex][neighbor] == 0:
                continue

            graph[vertex][neighbor] -= 1
            graph[neighbor][vertex] -= 1
            max_strength = max(max_strength, 2 * vertex + dfs(graph, neighbor))
            graph[vertex][neighbor] += 1
            graph[neighbor][vertex] += 1

        return max_strength

    return dfs(graph, 0)


def part2(graph):
    def dfs(graph, vertex):
        max_length = 0
        max_strength = 0

        for neighbor in graph[vertex].keys():
            # If we have no edge left with this neighbor, skip it
            if graph[vertex][neighbor] == 0:
                continue

            # Remove current edge
            graph[vertex][neighbor] -= 1
            graph[neighbor][vertex] -= 1

            max_neighbor_length, max_neighbor_strength = dfs(graph, neighbor)

            # Take into account the current edge, vertex -> neighbor
            max_neighbor_length += 1
            max_neighbor_strength += vertex + neighbor

            # Update max length and strength
            if max_neighbor_length == max_length:
                max_strength = max(max_strength, max_neighbor_strength)
            elif max_neighbor_length > max_length:
                max_length = max_neighbor_length
                max_strength = max_neighbor_strength

            # Put back current edge
            graph[vertex][neighbor] += 1
            graph[neighbor][vertex] += 1

        return max_length, max_strength

    _, strength = dfs(graph, 0)
    return strength


def solve(data):
    graph = parse_data(data)
    return part1(graph), part2(graph)


if __name__ == "__main__":
    puzzle = Puzzle(year=2017, day=24)

    for i, example in enumerate(puzzle.examples):
        answer_a, answer_b = solve(example.input_data)

        if example.answer_a is not None and answer_a is not None:
            assert answer_a == int(example.answer_a)
            print(f"Example {i} part 1: OK")

        if example.answer_b is not None and answer_b is not None:
            assert answer_b == int(example.answer_b)
            print(f"Example {i} part 2: OK")

    print(solve(data))
