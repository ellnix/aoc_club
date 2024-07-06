use itertools::Itertools;

use crate::input::day_4::INPUT;

pub fn part_1() -> usize {
    valid_count(false)
}

pub fn part_2() -> usize {
    valid_count(true)
}

fn valid_count(hardened: bool) -> usize {
    INPUT
        .iter()
        .filter(|passphrase| {
            !passphrase
                .split_ascii_whitespace()
                .map(|word| match hardened {
                    true => word.chars().sorted_unstable().collect(),
                    false => word.to_string(),
                })
                .counts()
                .into_values()
                .any(|count| count > 1)
        })
        .count()
}
