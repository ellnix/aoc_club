use itertools::Itertools;

use crate::input::day_2::INPUT; // Two-dimensional array of i32's

pub fn part_1() -> i32 {
    INPUT
        .iter()
        .map(|line| {
            let (min, max) = line.iter().minmax().into_option().unwrap();
            max - min
        })
        .sum()
}

pub fn part_2() -> i32 {
    INPUT
        .iter()
        .map(|line| {
            line.iter()
                .combinations(2)
                .filter_map(|c| {
                    let (&min, &max) = c.iter().minmax().into_option().unwrap();
                    (max % min == 0).then_some(max / min)
                })
                .exactly_one()
                .unwrap()
        })
        .sum()
}
