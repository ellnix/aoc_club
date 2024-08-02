use std::cell::RefCell;
use std::fmt;
use std::mem::swap;
use std::ops::{BitXor, DerefMut};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KnotHash {
    dense: Vec<u8>,
}

impl KnotHash {
    const EXTRA_LENGTHS: [u8; 5] = [17, 31, 73, 47, 23];

    pub fn new<I>(input: I) -> Self
    where
        I: AsRef<str>,
    {
        let lengths = Self::lengths(input);
        let sparse = Self::sparse_hash(lengths, 64);
        let dense = Self::dense_hash(sparse);

        Self { dense }
    }

    pub fn dense(&self) -> &Vec<u8> {
        &self.dense
    }

    pub fn lengths<I>(input: I) -> Vec<u8>
    where
        I: AsRef<str>,
    {
        input
            .as_ref()
            .as_bytes()
            .iter()
            .chain(&Self::EXTRA_LENGTHS)
            .cloned()
            .collect_vec()
    }

    pub fn sparse_hash(lengths: Vec<u8>, rounds: usize) -> Vec<u8> {
        let numbers = Self::initial_numbers();

        let num_lengths = lengths.len() * rounds;
        let _ = lengths
            .iter()
            .cycle()
            .take(num_lengths)
            .map(|&length| length as usize)
            .fold((numbers.iter().cycle(), 0usize), |(numbers, skip), length| {
                Self::swap_range(numbers.clone().take(length));
                (numbers.dropping(length + skip), skip + 1)
            });

        numbers.into_iter().map(RefCell::into_inner).collect()
    }

    pub fn dense_hash(sparse: Vec<u8>) -> Vec<u8> {
        sparse
            .into_iter()
            .chunks(16)
            .into_iter()
            .map(|chunk| chunk.into_iter().reduce(BitXor::bitxor).unwrap())
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
}

impl fmt::Display for KnotHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.dense.iter().map(|n| format!("{n:02x}")).join(""))
    }
}
