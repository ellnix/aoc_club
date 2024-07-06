#!/usr/bin/awk -f

#@include "../lib/aoc_lib.awk"
# function added to ../lib/aoc_lib.awk this week:

function min(a, b) {
    if (a < b) {
        return a
    }
    return b
}

function max(a, b) {
    if (a > b) {
        return a
    }
    return b
}
# end of ../lib/aoc_lib.awk

function solve_part_1(       minimum, maximum, i) {
    maximum = $1
    minimum = $1
    for (i = 2; i <= NF; i++) {
        minimum = min($i, minimum)
        maximum = max($i, maximum)
    }
    return maximum - minimum
}

function solve_part_2(      i, j) {
    for (i = 1; i<= NF; i++) {
        for (j = i + 1; j<= NF; j++) {
            if ($i % $j == 0) {
                return $i / $j
            } else if ($j % $i == 0 ) {
                return $j / $i
            }
        } 
    }
}

{
    total_part_1 += solve_part_1()   
    total_part_2 += solve_part_2()
}
END {
    print "Part 1: " total_part_1
    print "Part 2: " total_part_2
}