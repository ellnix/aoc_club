#!/usr/bin/awk -f

function solve(digits, last, i, j,     total) {
    total = 0
    while (i <= last) {
        if (digits[i] == digits[j]) {
            total += digits[i]
        }
        i++
        j++
        if (j > last) {
            j = 1
        }
    }
    return total
}

{
    split($0, digits, "")
    last = length($0)
    j_part_2 = last / 2 + 1
    total_part_1 = solve(digits, last, 1, 2)
    total_part_2 = solve(digits, last, 1, j_part_2)
}

END {
    print "Part 1: " total_part_1
    print "Part 2: " total_part_2
}