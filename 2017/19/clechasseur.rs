use std::iter::successors;

use itertools::Itertools;

use crate::helpers::direction::Direction;
use crate::helpers::pt::Pt;
use crate::helpers::turtle::Turtle;
use crate::input::day_19::INPUT;

pub fn part_1() -> String {
    packet().filter_map(|(_, c)| c.map(char::from)).collect()
}

pub fn part_2() -> usize {
    packet().count()
}

fn blockade(dir: Direction) -> u8 {
    match dir {
        Direction::Left | Direction::Right => b'|',
        Direction::Up | Direction::Down => b'-',
    }
}

fn packet() -> impl Iterator<Item = (Turtle<i64>, Option<u8>)> {
    let input: Vec<_> = INPUT.lines().collect();

    let start_x = input[0].bytes().find_position(|&c| c == b'|').unwrap().0;
    let start_pos = Pt::new(start_x as i64, 0);
    let turtle = Turtle::new(start_pos, Direction::Down);

    let at =
        move |t: &Turtle<i64>| input[(-t.position.y) as usize].as_bytes()[t.position.x as usize];

    successors(Some((turtle, None)), move |(turtle, _)| {
        Some(turtle.advance())
            .filter(|t| at(t) != b' ')
            .or_else(|| {
                Some(turtle.turn_left().advance())
                    .filter(|t| {
                        // Note: if I remove the test for `blockade`, it still works with my data.
                        let new_c = at(t);
                        new_c != b' ' && new_c != blockade(t.direction)
                    })
                    .or_else(|| Some(turtle.turn_right().advance()))
            })
            .map(|t| (t, at(&t)))
            .filter(|&(_, c)| c != b' ')
            .map(|(t, c)| (t, Some(c).filter(u8::is_ascii_uppercase)))
    })
}
