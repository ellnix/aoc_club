use std::iter::successors;

use crate::input::day_15::initial_values;

pub fn part_1() -> usize {
    matching_pairs(generator_a(), generator_b(), 40_000_000)
}

pub fn part_2() -> usize {
    let a = generator_a().picky(4);
    let b = generator_b().picky(8);

    matching_pairs(a, b, 5_000_000)
}

const THRESHOLD: u64 = i32::MAX as u64;

fn generator(initial_value: u64, factor: u64) -> impl Iterator<Item = u64> {
    successors(Some(initial_value), move |prev| Some(prev * factor % THRESHOLD)).skip(1)
}

trait Picky {
    fn picky(self, multiple_of: u64) -> impl Iterator<Item = u64>;
}

impl<T> Picky for T
where
    T: Iterator<Item = u64>,
{
    fn picky(self, multiple_of: u64) -> impl Iterator<Item = u64> {
        self.filter(move |&value| value % multiple_of == 0)
    }
}

fn generator_a() -> impl Iterator<Item = u64> {
    generator(initial_values::GENERATOR_A, 16_807)
}

fn generator_b() -> impl Iterator<Item = u64> {
    generator(initial_values::GENERATOR_B, 48_271)
}

fn matching_pairs<A, B>(a: A, b: B, rounds: usize) -> usize
where
    A: Iterator<Item = u64>,
    B: Iterator<Item = u64>,
{
    a.zip(b)
        .take(rounds)
        .filter(|&(a, b)| (a & 0xffff) == (b & 0xffff))
        .count()
}
