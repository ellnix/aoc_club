use std::iter::FusedIterator;
use std::vec;

use itertools::Itertools;

pub trait LoopingItertools<T> {
    fn looping(self, size: usize) -> Looping<T>;
}

impl<T, I> LoopingItertools<T> for I
where
    T: Eq + Clone,
    I: Iterator<Item = T>,
{
    fn looping(self, size: usize) -> Looping<T> {
        let mut prefix = Vec::new();

        for e in self {
            match prefix.iter().find_position(|&ve| *ve == e) {
                Some((start, _)) => {
                    let cycle = prefix.split_off(start);
                    return Looping::new(prefix, cycle, size);
                },
                None => prefix.push(e),
            }
        }

        panic!("no loop detected");
    }
}

#[derive(Debug, Clone)]
pub struct Looping<T> {
    prefix_size: usize,
    cycle_size: usize,
    prefix: vec::IntoIter<T>,
    prefix_pos: usize,
    cycle: Vec<T>,
    cycle_pos: usize,
}

impl<T> Looping<T> {
    pub fn new(prefix: Vec<T>, cycle: Vec<T>, size: usize) -> Self {
        Self {
            prefix_size: prefix.len(),
            cycle_size: size.saturating_sub(prefix.len()),
            prefix: prefix.into_iter(),
            prefix_pos: 0,
            cycle,
            cycle_pos: 0,
        }
    }
}

impl<T> Iterator for Looping<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.prefix.next() {
            Some(e) => {
                self.prefix_pos += 1;
                Some(e)
            },
            None if self.cycle_pos == self.cycle_size => None,
            None => {
                let e = self.cycle[self.cycle_pos % self.cycle.len()].clone();
                self.cycle_pos += 1;
                Some(e)
            },
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let last_pos_in_cycle = (self.cycle_size - 1) % self.cycle.len();
        self.cycle
            .into_iter()
            .nth(last_pos_in_cycle)
            .or_else(|| self.prefix.last())
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match ((self.prefix_pos + n) < self.prefix_size, (self.cycle_pos + n) < self.cycle_size) {
            (true, _) => {
                self.prefix_pos += n;
                self.prefix.nth(n)
            },
            (false, true) => {
                self.cycle_pos += n;
                Some(self.cycle[self.cycle_pos % self.cycle.len()].clone())
            },
            (false, false) => None,
        }
    }
}

impl<T> ExactSizeIterator for Looping<T>
where
    T: Clone,
{
    fn len(&self) -> usize {
        (self.prefix_size - self.prefix_pos) + (self.cycle_size - self.cycle_pos)
    }
}

impl<T> FusedIterator for Looping<T> where T: Clone {}
