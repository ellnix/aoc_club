use std::iter::successors;

use crate::input::day_5::INPUT;

pub fn part_1() -> usize {
    steps(false)
}

pub fn part_2() -> usize {
    steps(true)
}

fn steps(strange: bool) -> usize {
    // Skip the initial state, but count the last jump.
    maze(strange).skip(1).count() + 1
}

fn maze(strange: bool) -> impl Iterator<Item = usize> {
    let mut jumps: Vec<_> = INPUT.into();

    successors(Some(0_usize), move |&prev| {
        let jmp = jumps.get_mut(prev).unwrap();
        let next = prev.wrapping_add_signed(*jmp);
        *jmp += if strange && *jmp >= 3 { -1 } else { 1 };
        (next < jumps.len()).then_some(next)
    })
}
