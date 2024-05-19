use crate::input::day_1::INPUT;

pub fn part_1() -> u32 {
    sum(1)
}

pub fn part_2() -> u32 {
    sum(INPUT.len() / 2)
}

fn sum(skip: usize) -> u32 {
    INPUT
        .chars()
        .enumerate()
        .filter(|&(i, c)| c == nth(i + skip))
        .map(|(_, c)| c.to_digit(10).unwrap())
        .sum()
}

fn nth(i: usize) -> char {
    INPUT.chars().nth(i % INPUT.len()).unwrap()
}
