#!/usr/bin/awk -f

function solve_part_1(     offset) {
    offset = 1
    steps = 0
    while (offset in trampolines_part_1) {
        next_jump = trampolines_part_1[offset]
        #print "Step " steps ": At " offset ", about to jump " next_jump
        trampolines_part_1[offset]++
        offset += next_jump
        steps++
    }
    return steps
}

function solve_part_2(    offset) {
    offset = 1
    steps = 0
    while (offset in trampolines_part_2) {
        next_jump = trampolines_part_2[offset]
        if (next_jump >= 3) {
            trampolines_part_2[offset]--
        } else {
            trampolines_part_2[offset]++
        }
        offset += next_jump
        steps++
    }
    return steps
}

BEGIN {
    i = 1
}

# Skip the blank line at the end of the file
/[0-9]/ {
    trampolines_part_1[i] = int($1)
    trampolines_part_2[i] = int($1)
    i++
}

END {
    print "Part 1: " solve_part_1()
    print "Part 2: " solve_part_2()
}