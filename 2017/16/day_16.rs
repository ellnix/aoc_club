use std::iter::successors;
use std::str::FromStr;

use anyhow::{anyhow, Context};
use itertools::Itertools;

use crate::helpers::r#loop::Loop;
use crate::input::day_16::INPUT;

pub fn part_1() -> String {
    moves()
        .into_iter()
        .fold(initial_programs(), |programs, mv| mv.apply(programs))
        .into_iter()
        .collect()
}

pub fn part_2() -> String {
    let fake_times = 1_000_000_000usize;
    let moves = moves();
    let num_moves = moves.len();

    let dances = moves
        .clone()
        .into_iter()
        .cycle()
        .take(num_moves * fake_times)
        .chunks(num_moves);
    let mut dances = dances.into_iter();

    let dance_results = successors(Some(initial_programs()), move |programs| match dances.next() {
        Some(dance) => {
            let programs = dance.fold(programs.clone(), |programs, mv| mv.apply(programs));
            Some(programs)
        },
        None => None,
    });

    let looop = Loop::find(dance_results).expect("no loop detected!");
    let real_times = (fake_times - looop.start) % looop.len();
    moves
        .into_iter()
        .cycle()
        .take(num_moves * real_times)
        .fold(looop.duplicate, |programs, mv| mv.apply(programs))
        .into_iter()
        .collect()
}

fn initial_programs() -> Vec<char> {
    ('a'..='p').collect_vec()
}

fn moves() -> Vec<Move> {
    INPUT.split(',').map(|s| s.parse().unwrap()).collect_vec()
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
