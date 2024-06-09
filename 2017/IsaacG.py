#!/bin/python
import collections


class Day07(aoc.Challenge):

    INPUT_PARSER = aoc.parse_re_findall_mixed(r"\d+|[a-z]+")
    PARAMETERIZED_INPUTS = [False, True]

    def solver(self, parsed_input: list[list[int | str]], param: bool) -> int | str:
        node_weight: dict[str, int] = {}
        node_children: dict[str, set[str]] = collections.defaultdict(set)
        not_root: set[str] = set()

        node: str
        weight: int
        children: list[str]
        for (node, weight, *children) in parsed_input:  # type: ignore
            node_weight[node] = weight
            node_children[node].update(children)
            not_root.update(children)
        root = (set(node_weight) - not_root).pop()
        if not param:
            return root

        tower_weights: dict[str, int] = {}

        def sum_weights(node: str) -> int:
            """Compute weight of a tower recursively with dynamic programming."""
            if node not in tower_weights:
                tower_weights[node] = node_weight[node] + sum(
                    sum_weights(child) for child in node_children[node]
                )
            return tower_weights[node]

        to_check = set(node_weight)
        while to_check:
            # Find a node at the edge, i.e. without any children that haven't been explored.
            node = next(
                node
                for node in to_check
                if all(child not in to_check for child in node_children[node])
            )
            to_check.remove(node)

            # Find a tower with children of uneven weights.
            weights = collections.Counter(
                sum_weights(child) for child in node_children[node]
            ).most_common()
            if len(weights) <= 1:
                continue
            # Find which child is the outlier.
            (common, _), (outlier, _) = weights
            tower = next(child for child in node_children[node] if sum_weights(child) == outlier)

            return node_weight[tower] + (common - outlier)
        raise RuntimeError("No solution found.")
