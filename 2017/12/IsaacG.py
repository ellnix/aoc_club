PARAMETERIZED_INPUTS = [True, False]
INPUT_PARSER = aoc.parse_ints_per_line

def solver(parsed_input: list[list[int]], part_one: bool) -> int:
    # Build a bidirectional map of pipes.
    pipes = collections.defaultdict(set)
    for one, *others in parsed_input:
        for other in others:
            pipes[one].add(other)
            pipes[other].add(one)

    group_sizes = {}
    # Loop until all processes have been handled.
    unseen = set(pipes)
    while unseen:
        # Pop one process to explore a group. Use min() to ensure we have a "0" group.
        # Expand that group until we've seen all neighbors.
        seed = min(unseen)
        todo = {seed}
        seen = set()
        while todo:
            cur = todo.pop()
            unseen.remove(cur)
            seen.add(cur)
            for other in pipes[cur]:
                if other not in seen:
                    todo.add(other)
        # The number of seen processes is the group size.
        group_sizes[seed] = len(seen)
    # The len(group_sizes) is the number of groups.
    return group_sizes[0] if part_one else len(group_sizes)
