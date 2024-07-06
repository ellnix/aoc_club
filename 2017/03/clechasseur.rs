use std::collections::HashMap;
use std::iter;

use itertools::Itertools;
use num::{zero, Zero};

use crate::helpers::direction::Direction;
use crate::helpers::pt::{manhattan, Pt};
use crate::input::day_3::INPUT;

pub fn part_1() -> i64 {
    manhattan(zero(), spiral().nth(INPUT - 1).unwrap())
}

pub fn part_2() -> i64 {
    spiral_stress_test().find(|&v| v > (INPUT as i64)).unwrap()
}

fn spiral() -> impl Iterator<Item = Pt<i64>> {
    let mut pt = zero();
    let mut max_moves = 1;
    let mut moves = 1;
    let mut times = 2;
    let mut direction = Direction::Right;

    iter::repeat_with(move || {
        let this_pt = pt;

        pt += direction.displacement();
        moves -= 1;
        if moves == 0 {
            direction = direction.turn_left();
            times -= 1;
            if times == 0 {
                max_moves += 1;
                times = 2;
            }
            moves = max_moves;
        }

        this_pt
    })
}

fn spiral_stress_test() -> impl Iterator<Item = i64> {
    let mut values = HashMap::new();
    let around: Vec<_> = (-1i64..=1)
        .cartesian_product(-1i64..=1)
        .map(Into::<Pt<_>>::into)
        .filter(|pt| !pt.is_zero())
        .collect();

    spiral().map(move |pt| {
        let value = around
            .iter()
            .filter_map(|&pt_mod| {
                let neighbour = pt + pt_mod;
                values.get(&neighbour).copied()
            })
            .sum1()
            .unwrap_or(1i64);
        values.insert(pt, value);
        value
    })
}
