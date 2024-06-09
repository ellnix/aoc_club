#!/usr/bin/awk -f

@include "/usr/share/awk/join.awk"

function get_max(row,    highest, highest_seen, highest_idx, i) {
    highest_seen=0
    highest_idx=1
    split(row, row_arr)
    for (i in row_arr) {
        if (row_arr[i] > highest_seen) {
            highest_idx = i
            highest_seen = row_arr[i]
        }
    }
    highest = highest_idx " " highest_seen
    return highest
}

function redistribute_blocks(     row, idx, amount_to_redistribute) {
    split(get_max(row), highest)
    idx = highest[1]
    amount_to_redistribute = highest[2]
    split(row, row_arr)
    row_arr[idx] = 0
    while (amount_to_redistribute > 0) {
        idx++
        if (idx > NF) {
            # arrays are one indexed so modular arithmetic is more awkward than this
            idx = 1
        }
        row_arr[idx] += 1
        amount_to_redistribute--
    }
    row = join(row_arr, 1, NF)
    return row
}

function solve_day_6(    steps, configuration, joined_configuration) {
    steps = 0
    configuration = $0
    joined_configuration = configuration
    gsub(/ /, "_", joined_configuration)
    while (!(joined_configuration in cache)) {
        cache[joined_configuration]=steps
        configuration = redistribute_blocks(configuration)
        joined_configuration = configuration
        gsub(/ /, "_", joined_configuration)
        steps++
    }
    print "Part 1: " steps
    print "Part 2: " steps - cache[joined_configuration]
}
{
    solve_day_6()
}
