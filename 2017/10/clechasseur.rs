use std::cell::RefCell;
use std::mem::swap;
use std::ops::{BitXor, DerefMut, Mul};

use itertools::Itertools;

use crate::input::day_10::INPUT;

pub fn part_1() -> usize {
    sparse_hash(&part_1_lengths(), 1)
        .into_iter()
        .map(|n| n as usize)
        .take(2)
        .reduce(Mul::mul)
        .unwrap()
}

pub fn part_2() -> String {
    knot_hash(INPUT)
}

fn part_1_lengths() -> Vec<u8> {
    INPUT
        .split(',')
        .map(|length| length.parse().unwrap())
        .collect()
}

fn initial_numbers() -> Vec<RefCell<u8>> {
    (u8::MIN..=u8::MAX).map(RefCell::new).collect()
}

fn swap_range<'a, I, T>(range: I)
where
    I: Iterator<Item = &'a RefCell<T>>,
    T: 'a,
{
    let mut range = range.collect_vec().into_iter();

    while let (Some(a), Some(b)) = (range.next(), range.next_back()) {
        swap(a.borrow_mut().deref_mut(), b.borrow_mut().deref_mut());
    }
}

fn sparse_hash(lengths: &[u8], rounds: usize) -> Vec<u8> {
    let numbers = initial_numbers();

    let num_lengths = lengths.len() * rounds;
    let _ = lengths
        .iter()
        .cycle()
        .take(num_lengths)
        .map(|&length| length as usize)
        .fold((numbers.iter().cycle(), 0usize), |(numbers, skip), length| {
            swap_range(numbers.clone().take(length));
            (numbers.dropping(length + skip), skip + 1)
        });

    numbers.into_iter().map(RefCell::into_inner).collect()
}

fn dense_hash(sparse: Vec<u8>) -> Vec<u8> {
    sparse
        .into_iter()
        .chunks(16)
        .into_iter()
        .map(|chunk| chunk.into_iter().reduce(BitXor::bitxor).unwrap())
        .collect()
}

fn to_hex_string(dense: Vec<u8>) -> String {
    dense.into_iter().map(|n| format!("{n:02x}")).join("")
}

const EXTRA_LENGTHS: &[u8] = &[17, 31, 73, 47, 23];

fn knot_hash(input: &str) -> String {
    let lengths = input
        .as_bytes()
        .iter()
        .chain(EXTRA_LENGTHS)
        .cloned()
        .collect_vec();

    let sparse = sparse_hash(&lengths, 64);
    let dense = dense_hash(sparse);
    to_hex_string(dense)
}
