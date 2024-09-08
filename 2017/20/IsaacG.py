import collections

PARAMETERIZED_INPUTS = [False, True]
INPUT_PARSER = aoc.parse_ints_per_line


def distance(vals: tuple[int, ...]) -> int:
    """Return the Manhatten distance of a coordinate."""
    return sum(abs(i) for i in vals)


def solver(parsed_input: list[list[int]], part_two: bool) -> int:
    """Return particle metadata.

    Part one: return which particle will be farthest from the origin.
    Part two: return the number of particles after collisions.
    """
    pos, vel, acc = [
        {idx: tuple(vals[i * 3:(i + 1) * 3]) for idx, vals in enumerate(parsed_input)}
        for i in range(3)
    ]
    # Simulate some ticks for collision removal. 40 is the min that works for my input.
    for _ in range(100 if part_two else 0):
        # Count positions. Any position with multiple particles is a collision.
        counts = collections.Counter(pos.values())
        collisions = {p for p, count in counts.items() if count > 1}
        # Remove particles which collided.
        pos = {idx: p for idx, p in pos.items() if p not in collisions}
        vel = {idx: v for idx, v in vel.items() if idx in pos}
        # Update velocity then position.
        vel = {idx: tuple(vel[idx][dim] + acc[idx][dim] for dim in range(3)) for idx in pos}
        pos = {idx: tuple(pos[idx][dim] + vel[idx][dim] for dim in range(3)) for idx in pos}

    if part_two:
        return len(pos)
    # Sort remaining particles by acceleration then velocity (Manhatten distance).
    slowest = sorted((self.distance(acc[idx]), self.distance(vel[idx]), idx) for idx in pos)
    return slowest[0][2]
