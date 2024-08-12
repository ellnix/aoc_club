use std::iter::successors;
use std::str::FromStr;

use anyhow::{anyhow, Context};
use itertools::Itertools;

use crate::helpers::r#loop::Loop;
use crate::input::day_16::INPUT;

pub fn part_1() -> String {
    dance(&moves(), initial_programs()).to_string_please()
}

pub fn part_2() -> String {
    let moves = moves();
    let fake_times = 1_000_000_000usize;

    let dances =
        successors(Some(initial_programs()), |programs| Some(dance(&moves, programs.clone())))
            .take(fake_times);

    let r#loop = Loop::find(dances).expect("no loop detected!");
    r#loop.last_from_total(fake_times).to_string_please()
}

fn initial_programs() -> Vec<char> {
    ('a'..='p').collect_vec()
}

fn moves() -> Vec<Move> {
    INPUT.split(',').map(|s| s.parse().unwrap()).collect_vec()
}

fn dance(moves: &[Move], programs: Vec<char>) -> Vec<char> {
    moves
        .iter()
        .fold(programs, |programs, mv| mv.apply(programs))
}

trait ToStringPlease {
    fn to_string_please(&self) -> String;
}

impl ToStringPlease for Vec<char> {
    fn to_string_please(&self) -> String {
        self.iter().collect()
    }
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Move {
    pub fn apply(self, mut programs: Vec<char>) -> Vec<char> {
        match self {
            Self::Spin(len) => programs
                .iter()
                .tail(len)
                .chain(programs.iter().dropping_back(len))
                .copied()
                .collect_vec(),
            Self::Exchange(pos_a, pos_b) => {
                programs.swap(pos_a, pos_b);
                programs
            },
            Self::Partner(par_a, par_b) => {
                let pos_a = programs.iter().position(|&c| c == par_a).unwrap();
                let pos_b = programs.iter().position(|&c| c == par_b).unwrap();
                programs.swap(pos_a, pos_b);
                programs
            },
        }
    }
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let invalid_move = || format!("invalid move: {s}");

        match s.chars().next() {
            Some(mv) => match mv {
                's' => Ok(Self::Spin(s[1..].parse()?)),
                'x' => {
                    let (a, b) = s[1..].split_once('/').with_context(invalid_move)?;
                    Ok(Self::Exchange(a.parse()?, b.parse()?))
                },
                'p' => {
                    let (a, b) = s[1..].split_once('/').with_context(invalid_move)?;
                    Ok(Self::Partner(
                        a.chars().next().with_context(invalid_move)?,
                        b.chars().next().with_context(invalid_move)?,
                    ))
                },
                _ => Err(anyhow!("invalid move: {mv}")),
            },
            None => Err(anyhow!("empty move string")),
        }
    }
}
