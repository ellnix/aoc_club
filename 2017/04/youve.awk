#!/usr/bin/awk -f

@include "../lib/aoc_lib.awk" # unused this week
@include "/usr/share/awk/join.awk"

# Part 1
function all_words_are_unique() {
    delete words
    for (i = 1; i <= NF; i++) {
        if (words[$i] == 1) {
            return 0
        }
        words[$i]++
    }
    return 1
}

# Part 2
function all_anagrams_are_unique(    sorted) {
    # empty the dictionary so previous rows won't affect this row
    delete words
    for (i = 1; i <= NF; i++) {
        # turn the word into an array ["h", "e", "l", "l", "o"]
        split($i, split_i, "")
        # sort the array alphabetically, and record its length: ["e", "h", "l", "l", "o"]
        len = asort(split_i, a_sorted)
        # join the array back up into a string ["ehllo"]
        sorted = join(a_sorted, 1, len, SUBSEP)
        if (words[sorted] == 1) {
            # if we've seen this word already, return 0
            return 0
        }
        # otherwise put the word in the dictionary and increment it.
        words[sorted]++
    }
    # all words are unique
    return 1
}

{
    part_1_total += all_words_are_unique()
    part_2_total += all_anagrams_are_unique()
}

END {
    print "Part 1: " part_1_total
    print "Part 2: " part_2_total
}