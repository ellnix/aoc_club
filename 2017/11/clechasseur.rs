use std::iter::successors;

use num::Zero;
use strum::{EnumProperty, EnumString};

use crate::helpers::pt::Pt;
use crate::input::day_11::INPUT;

pub fn part_1() -> usize {
    distance_to(child_position())
}

pub fn part_2() -> usize {
    child_path().map(distance_to).max().unwrap()
}

fn child_path() -> impl Iterator<Item = Pt<isize>> {
    INPUT
        .split(',')
        .map(|dir| dir.parse::<HexDirection>().unwrap())
        .scan(Pt::zero(), |pt, dir| {
            *pt += dir.displacement();
            Some(*pt)
        })
}

fn child_position() -> Pt<isize> {
    child_path().last().unwrap()
}

fn distance_to(goal: Pt<isize>) -> usize {
    path_to(goal).count() - 1
}

fn path_to(goal: Pt<isize>) -> impl Iterator<Item = Pt<isize>> {
    successors(Some(Pt::<isize>::zero()), move |&pt| {
        match ((goal.x - pt.x).signum(), (goal.y - pt.y).signum()) {
            (0, 0) => None,
            (0, y) => Some(pt + Pt::new(0, y * 2)),
            (x, y) => Some(pt + Pt::new(x, y)),
        }
    })
}

#[derive(Debug, Copy, Clone, EnumProperty, EnumString)]
#[strum(serialize_all = "snake_case")]
enum HexDirection {
    #[strum(props(displacement = "(-1, 1)"))]
    NW,
    #[strum(props(displacement = "(0, 2)"))]
    N,
    #[strum(props(displacement = "(1, 1)"))]
    NE,
    #[strum(props(displacement = "(1, -1)"))]
    SE,
    #[strum(props(displacement = "(0, -2)"))]
    S,
    #[strum(props(displacement = "(-1, -1)"))]
    SW,
}

impl HexDirection {
    fn displacement(&self) -> Pt<isize> {
        // Note: here, `parse` calls `impl FromStr for Pt`, which
        // is implemented as parsing points in format `(x, y)`.
        self.get_str("displacement").unwrap().parse().unwrap()
    }
}
