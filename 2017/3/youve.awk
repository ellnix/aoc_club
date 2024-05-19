#!/usr/bin/awk -f

#@include "../lib/aoc_lib.awk"
# function added to ../lib/aoc_lib.awk this week:

function ceil(n) {
    ival = int(n)
    if (ival == n) {
        return n
    }
    if (n > 0) {
        return ival + 1
    }
    return ival
}
# end of ../lib/aoc_lib.awk

# Part 1:

# 17  16  15  14  13
# 18   5   4   3  12
# 19   6   1   2  11
# 20   7   8   9  10
# 21  22  23  24  25

function solve_part_1(n) {
    # Almost O(1) solution except for the sqrt
    side_length = ceil(sqrt(n)) - 1  # takes us to the northeast or southeast corner
    if (side_length % 2 != 0) {
        side_length += 1 # takes us to the southeast corner
    }
    half_side = side_length / 2
    overshot = (side_length + 1 ) ** 2 - n
    if (overshot == 0) {
        return side_length
    }
    south = east = half_side
    while (overshot > side_length) {
        #rotational symmetry, this will complete in at most 3 iterations.
        overshot -= side_length
    }
    if (overshot > half_side) {
        # mirror symmetry
        overshot = side_length - overshot
    }
    return south + east - overshot
}

# Functions for part 2:

# 147  142  133  122   59
# 304    5    4    2   57
# 330   10    1    1   54
# 351   11   23   25   26
# 362  747  806  880  931

function get_neighbours(          i, j, result) {
    # Awk cannot return an array, so our choices are to return a string
    # Or to manipulate a global array. This time, we're returning a string
    # Formatted like i1,j1 i2,j2 i3,j3 ... i8,j8
    for (i = -1; i <= 1; i++) {
        for (j = -1; j <= 1; j++) {
            result = turtle["x"] + i "," turtle["y"] + j " " result
        }
    }
    return result
}

function sum_neighbours(n,    total, x, y, value, i) {
    # This returns a number
    total = 0
    # This turns the output of get_neighbours into the array we wanted.
    split(get_neighbours(turtle["x"], turtle["y"]), neighbours)
    for (i in neighbours) {
        # Parse string i,j into array [i, j]
        split(neighbours[i], x_y, ",")
        x = x_y[1]
        y = x_y[2]
        total += spiral[x, y]
    }
    if (total > n) {
        print "Part 2: " total
        exit 1
    }
    return total
}

function update_turtle_coords_and_direction() {
    # This updates a struct (really an array but it's a struct in my head)
    # Instead of returning something
    # Because string variables touched inside a function
    # Are local, and array variables touched inside a function
    # Are global. And we can only return a string or nothing
    if (turtle["direction"] == "N") {
        turtle["y"]++
    } else if (turtle["direction"] == "E") {
        turtle["x"]++
    } else if (turtle["direction"] == "S") {
        turtle["y"]--
    } else if (turtle["direction"] == "W") {
        turtle["x"]--
    }
    turtle["side_posi"]++
    if (turtle["side_length"] <= turtle["side_posi"]) {
        if (turtle["direction"] == "N" || turtle["direction"] == "S") {
            turtle["side_length"]++
        }
        turtle["side_posi"] = 0
        next_dir = next_direction[turtle["direction"]]
        turtle["direction"] = next_dir
    }
}

function solve_part_2(n,     i) {
    # Creates and modifies global variable because awk cannot return an array.
    for (i = 2; i <= n; i++) {
        spiral[turtle["x"], turtle["y"]] = sum_neighbours(n)
        update_turtle_coords_and_direction()
    }
}

BEGIN {
    next_direction["E"] = "N"
    next_direction["N"] = "W"
    next_direction["W"] = "S"
    next_direction["S"] = "E"
    # We're starting at number 2 with the first number filled out for us.
    turtle["direction"] = "N" # heading this way
    turtle["side_length"] = 1 # how long the current side
    turtle["side_posi"] = 0 # how far along the side
    turtle["x"] = 1
    turtle["y"] = 0
    spiral[0,0] = 1
}

{
    print "Part 1: " solve_part_1($0)
    print solve_part_2($0)
}

